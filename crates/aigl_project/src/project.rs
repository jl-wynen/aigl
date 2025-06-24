use anyhow::{Result, bail};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use super::config;
use aigl_git::Repository;
use aigl_system::fs::{copy_dir_recursive, create_output_directory};

pub struct Project {
    root: PathBuf,
    python_cache: aigl_python::Cache,
    cfg: config::project::ProjectConfig,
}

impl Project {
    pub async fn init(
        path: PathBuf,
        game_config: config::game::GameConfig,
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
                bot_template_path: PathBuf::new(),
                venv_paths: HashMap::new(),
            },
        }));

        set_up_repos(project.clone()).await?;

        // TODO create venvs
        // TODO install packages

        {
            let project = project.lock().expect("Failed to get project lock");
            project.save_config().await?;
            untag_dir_as_incomplete(&launcher_dir).await?;
        }
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

    let bots_dir = config::bot_templates_dir(&launcher_dir);
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

fn init_python_cache(launcher_dir: &Path) -> Result<aigl_python::Cache> {
    aigl_python::Cache::init(&config::uv_cache_dir(launcher_dir))
}

fn open_python_cache(launcher_dir: &Path) -> Result<aigl_python::Cache> {
    aigl_python::Cache::discover(&config::uv_cache_dir(launcher_dir))
}

fn clone_game_repo(join_set: &mut tokio::task::JoinSet<Result<()>>, project: Arc<Mutex<Project>>) {
    join_set.spawn_blocking(move || {
        let mut project = project.lock().expect("Failed to get project lock");
        let url = project.cfg.game_config.game.url.to_owned();
        let target = project.root.join(&project.cfg.game_config.name);
        match Repository::clone(&url, &target, false) {
            Ok(_) => {
                project.cfg.game_path = target.clone();
                Ok(())
            }
            Err(err) => Err(err),
        }
    });
}

fn clone_bot_template_repo(project: Arc<Mutex<Project>>) -> Result<()> {
    let mut project = project.lock().expect("Failed to get project lock");
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

async fn render_bot_template(project: Arc<Mutex<Project>>, target: &Path) -> Result<()> {
    let src = {
        let project = project.lock().expect("Failed to get project lock");
        project.cfg.bot_template_path.clone()
    };
    copy_dir_recursive(&src, target).await
}

fn set_up_initial_bots(
    join_set: &mut tokio::task::JoinSet<Result<()>>,
    project: Arc<Mutex<Project>>,
) {
    join_set.spawn(async move {
        let clone_project = project.clone();
        tokio::task::spawn_blocking(move || clone_bot_template_repo(clone_project)).await??;

        let mut tasks = tokio::task::JoinSet::<Result<()>>::new();
        tasks.spawn(async move {
            let target = {
                let lock = project.lock().expect("Failed to get project lock");
                lock.root.join("bot")
            };
            render_bot_template(project.clone(), &target).await?;
            Ok(())
        });
        // TODO render default bots
        tasks.join_all().await;
        Ok(())
    });
}

async fn set_up_repos(project: Arc<Mutex<Project>>) -> Result<()> {
    let mut tasks = tokio::task::JoinSet::new();
    set_up_initial_bots(&mut tasks, project.clone());
    clone_game_repo(&mut tasks, project.clone());
    match tasks
        .join_all()
        .await
        .into_iter()
        .find_map(|result| result.err())
    {
        None => Ok(()),
        Some(err) => Err(err),
    }
}
