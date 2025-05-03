use super::config;
use aigl_system::fs::create_output_directory;
use anyhow::{Result, bail};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Project {
    root: PathBuf,
    python_cache: aigl_python::Cache,
    cfg: config::project::ProjectConfig,
}

impl Project {
    pub async fn init(path: PathBuf, game_config: config::game::GameConfig) -> Result<Self> {
        create_output_directory(&path).await?;
        let launcher_dir = init_launcher_dir(&path).await?;
        let python_cache = init_python_cache(&launcher_dir)?;
        Ok(Self {
            root: path,
            python_cache,
            cfg: config::project::ProjectConfig {
                game_config,
                venv_locations: HashMap::new(),
            },
        })
    }

    pub fn open(path: PathBuf) -> Result<Self> {
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
            config::project::ProjectConfig::load_toml(&config::project_config_file(&launcher_dir))?;
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
                let venv_path = &self.cfg.venv_locations["game"];
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
}

async fn init_launcher_dir(project_root: &Path) -> Result<PathBuf> {
    let launcher_dir = config::launcher_dir(project_root);
    create_output_directory(&launcher_dir).await?;
    tokio::fs::write(launcher_dir.join(".gitignore"), "*").await?;

    let bots_dir = launcher_dir.join("bots");
    create_output_directory(&bots_dir).await?;
    cachedir::ensure_tag(&bots_dir)?;

    // TODO config

    Ok(launcher_dir)
}

fn init_python_cache(launcher_dir: &Path) -> Result<aigl_python::Cache> {
    aigl_python::Cache::init(&config::uv_cache_dir(launcher_dir))
}

fn open_python_cache(launcher_dir: &Path) -> Result<aigl_python::Cache> {
    aigl_python::Cache::discover(&config::uv_cache_dir(launcher_dir))
}
