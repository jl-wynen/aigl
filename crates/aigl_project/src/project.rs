use anyhow::{Result, bail};
use async_lock::Mutex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::bot::{Bot, BotArg, BotArgValue};
use crate::config;
use crate::unique_selection::UniqueRandomSelection;
use aigl_git::Repository;
use aigl_system::fs::create_output_directory;

pub struct Project {
    root: PathBuf,
    python_cache: aigl_python::Cache,
    cfg: config::project::ProjectConfig,
    name_gen: UniqueRandomSelection<String>,
    color_gen: UniqueRandomSelection<String>,
}

impl Project {
    pub async fn init(
        path: PathBuf,
        game_config: config::game::GameConfig,
        player_bot_id: String,
        player_bot_name: String,
        player_bot_args: Vec<BotArg>,
    ) -> Result<Arc<Mutex<Self>>> {
        create_output_directory(&path).await?;
        let launcher_dir = init_launcher_dir(&path).await?;
        let python_cache = init_python_cache(&launcher_dir)?;

        let project = Arc::new(Mutex::new(Self {
            root: path,
            python_cache,
            cfg: config::project::ProjectConfig {
                game_config,
                game_path: PathBuf::new(),
                bot_paths: Vec::new(),
                bot_template_path: PathBuf::new(),
                venv_paths: HashMap::new(),
            },
            name_gen: crate::bot_names::bot_name_selection(std::iter::empty::<String>()),
            color_gen: crate::bot_colors::bot_color_selection(std::iter::empty::<String>()),
        }));

        set_up_repos(
            project.clone(),
            player_bot_id,
            player_bot_name,
            player_bot_args,
        )
        .await?;
        create_venvs(project.clone()).await?;

        {
            let project = project.lock().await;
            project.save_config().await?;
        }
        untag_dir_as_incomplete(&launcher_dir).await?;
        Ok(project)
    }

    pub async fn open(path: PathBuf) -> Result<Self> {
        if !path.exists() {
            bail!("Project directory does not exist: {}", path.display());
        }
        let launcher_dir = config::launcher_dir(&path);
        if !path.is_dir() || !launcher_dir.exists() {
            bail!(
                "Project path does not look like an AI game directory: {}",
                path.display()
            );
        }

        let cfg =
            config::project::ProjectConfig::load_json(&config::project_config_file(&path)).await?;
        let python_cache = open_python_cache(&launcher_dir)?;
        Ok(Self {
            root: path,
            python_cache,
            cfg,
            // TODO load used names and colors
            name_gen: crate::bot_names::bot_name_selection(std::iter::empty::<String>()),
            color_gen: crate::bot_colors::bot_color_selection(std::iter::empty::<String>()),
        })
    }

    pub fn python_cache(&self) -> &aigl_python::Cache {
        &self.python_cache
    }

    pub fn venv_path(&self) -> Result<PathBuf> {
        match self.cfg.game_config.python.venv {
            config::game::VenvKind::Single => {
                let venv_path = &self.cfg.venv_paths["game"];
                Ok(self.root.join(venv_path))
            }
            _ => {
                bail!("No unique venv")
            }
        }
    }

    pub fn cfg(&self) -> &config::project::ProjectConfig {
        &self.cfg
    }

    pub fn cfg_mut(&mut self) -> &mut config::project::ProjectConfig {
        &mut self.cfg
    }

    pub fn venv(&self) -> Result<aigl_python::VirtualEnvironment> {
        aigl_python::VirtualEnvironment::open(self.venv_path()?.to_owned(), self.python_cache())
    }

    async fn save_config(&self) -> Result<()> {
        self.cfg
            .save_json(&config::project_config_file(&self.root))
            .await
    }
}

async fn init_launcher_dir(project_root: &Path) -> Result<PathBuf> {
    let launcher_dir = config::launcher_dir(project_root);
    create_output_directory(&launcher_dir).await?;
    tag_dir_as_incomplete(&launcher_dir).await?;
    tokio::fs::write(launcher_dir.join(".gitignore"), "*").await?;

    let bots_dir = config::bot_templates_dir(project_root);
    create_output_directory(&bots_dir).await?;
    cachedir::ensure_tag(&bots_dir)?;

    Ok(launcher_dir)
}

async fn tag_dir_as_incomplete(dir: &Path) -> Result<()> {
    tokio::fs::write(dir.join("aigl_in_progress"), "").await?;
    Ok(())
}

async fn untag_dir_as_incomplete(dir: &Path) -> Result<()> {
    tokio::fs::remove_file(dir.join("aigl_in_progress")).await?;
    Ok(())
}

pub fn dir_is_incomplete(dir: &Path) -> bool {
    dir.join("aigl_in_progress").exists()
}

fn init_python_cache(launcher_dir: &Path) -> Result<aigl_python::Cache> {
    aigl_python::Cache::init(&config::uv_cache_dir(launcher_dir))
}

fn open_python_cache(launcher_dir: &Path) -> Result<aigl_python::Cache> {
    aigl_python::Cache::discover(&config::uv_cache_dir(launcher_dir))
}

async fn clone_game_repo(project: Arc<Mutex<Project>>) -> Result<()> {
    let mut project = project.lock().await;
    let url = project.cfg.game_config.game.url.to_owned();
    let target = project.root.join(&project.cfg.game_config.name);
    match Repository::clone(&url, &target, false) {
        Ok(_) => {
            project.cfg.game_path = target.clone();
            Ok(())
        }
        Err(err) => Err(err),
    }
}

async fn clone_bot_template_repo(project: Arc<Mutex<Project>>) -> Result<()> {
    let mut project = project.lock().await;
    let url = project.cfg.game_config.bot.template_url.to_owned();
    let target = config::bot_templates_dir(&project.root).join("template");
    match Repository::clone(&url, &target, true) {
        Ok(_) => {
            project.cfg.bot_template_path = target.clone();
            Ok(())
        }
        Err(err) => Err(err),
    }
}

async fn set_up_initial_bots(
    project: Arc<Mutex<Project>>,
    player_bot_id: String,
    player_bot_name: String,
    player_bot_args: Vec<BotArg>,
) -> Result<()> {
    let clone_project = project.clone();
    clone_bot_template_repo(clone_project).await?;

    let n_bots = match project.lock().await.cfg.game_config.players {
        config::game::Players::FFA { n_min, .. } => n_min,
        config::game::Players::Teams { .. } => {
            todo!("Support for teams is not implemented")
        }
    };

    let mut tasks = tokio::task::JoinSet::<Result<Bot>>::new();
    let project_clone = project.clone();
    tasks.spawn(async move {
        render_player_bot(
            project_clone,
            player_bot_id,
            player_bot_name,
            player_bot_args,
        )
        .await
    });
    for i in 1..n_bots {
        let project_clone = project.clone();
        tasks.spawn(async move { render_template_bot(project_clone, format!("bot_{i}")).await });
    }
    tasks.join_all().await;
    Ok(())
}

async fn render_player_bot(
    project: Arc<Mutex<Project>>,
    bot_id: String,
    bot_name: String,
    args: Vec<BotArg>,
) -> Result<Bot> {
    let target = {
        let lock = project.lock().await;
        lock.root.join(&bot_id)
    };
    Bot::render_template(project.clone(), &target, bot_id, bot_name, args).await
}

async fn render_template_bot(project: Arc<Mutex<Project>>, bot_id: String) -> Result<Bot> {
    let (target, bot_name, args) = {
        let mut lock = project.lock().await;
        let mut args = Vec::new();
        for arg_spec in lock.cfg.game_config.bot.template_args.clone().values() {
            let arg_value = match &arg_spec.ty {
                config::game::BotTemplateArgType::Color => {
                    BotArgValue::color_from_string(&lock.color_gen.pop())
                }
                config::game::BotTemplateArgType::String => {
                    return Err(anyhow::anyhow!("String args are not supported yet"));
                }
                config::game::BotTemplateArgType::Path => {
                    return Err(anyhow::anyhow!("Path args are not supported yet"));
                }
            };
            args.push(BotArg {
                var: arg_spec.var.clone(),
                display: arg_spec.display.clone(),
                value: arg_value,
            });
        }
        (lock.root.join(&bot_id), lock.name_gen.pop(), args)
    };

    Bot::render_template(project.clone(), &target, bot_id, bot_name, args).await
}

async fn set_up_repos(
    project: Arc<Mutex<Project>>,
    player_bot_id: String,
    player_bot_name: String,
    player_bot_args: Vec<BotArg>,
) -> Result<()> {
    set_up_initial_bots(
        project.clone(),
        player_bot_id,
        player_bot_name,
        player_bot_args,
    )
    .await?;
    clone_game_repo(project.clone()).await?;
    Ok(())
}

async fn create_venvs(project: Arc<Mutex<Project>>) -> Result<()> {
    let mut lock = project.lock().await;
    let python_config = &lock.cfg.game_config.python;
    if matches!(python_config.venv, config::game::VenvKind::PerBot) {
        bail!("Per-bot virtual environments are not supported yet");
    };
    let venv_dir = lock.root.join(".venv");
    let venv = aigl_python::VirtualEnvironment::create(
        venv_dir.clone(),
        &python_config.version,
        &lock.python_cache,
    )
    .await?;

    let requirements: Vec<_> = lock
        .cfg()
        .bot_paths
        .iter()
        .map(|path| aigl_python::RequirementsSource::Editable(path.display().to_string()))
        .collect();
    let player_bot_path = lock.cfg().bot_paths.first().unwrap();
    let groups = [(player_bot_path.join("pyproject.toml"), "dev".into())];
    aigl_python::install(&requirements, groups, true, &lock.python_cache, &venv).await?;

    lock.cfg.venv_paths.insert("game".into(), venv_dir);
    Ok(())
}
