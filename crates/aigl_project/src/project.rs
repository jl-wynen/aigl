use super::config;
use anyhow::{Result, bail};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Project {
    path: PathBuf,
    python_cache: aigl_python::Cache,
    venv_locations: HashMap<String, PathBuf>,
}

impl Project {
    pub async fn init(path: PathBuf) -> Result<Self> {
        if path.exists() {
            bail!(
                "Cannot create project at {}: directory already exists",
                path.display()
            );
        }
        tokio::fs::create_dir_all(&path).await?;
        let launcher_dir = init_launcher_dir(&path).await?;
        let python_cache = init_python_cache(&launcher_dir)?;
        Ok(Self {
            path,
            python_cache,
            venv_locations: HashMap::new(),
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

        let python_cache = open_python_cache(&launcher_dir)?;
        Ok(Self {
            path,
            python_cache,
            venv_locations: HashMap::new(),
        })
    }

    pub fn python_cache(&self) -> &aigl_python::Cache {
        &self.python_cache
    }

    pub fn venv_path(&self) -> Result<&Path> {
        match self.venv_locations.len() {
            0 => bail!("No virtual environments"),
            1 => Ok(self.venv_locations.iter().next().unwrap().1),
            n => bail!("Multiple virtual environments: {}", n),
        }
    }

    pub fn venv(&self) -> Result<aigl_python::VirtualEnvironment> {
        aigl_python::VirtualEnvironment::open(self.venv_path()?.to_owned(), self.python_cache())
    }
}

async fn init_launcher_dir(project_root: &Path) -> Result<PathBuf> {
    let launcher_dir = config::launcher_dir(project_root);
    tokio::fs::create_dir(&launcher_dir).await?;
    tokio::fs::write(launcher_dir.join(".gitignore"), "*").await?;

    let bots_dir = launcher_dir.join("bots");
    tokio::fs::create_dir(&bots_dir).await?;
    cachedir::ensure_tag(&bots_dir)?;

    Ok(launcher_dir)
}

fn init_python_cache(launcher_dir: &Path) -> Result<aigl_python::Cache> {
    aigl_python::Cache::init(&config::uv_cache_dir(launcher_dir))
}

fn open_python_cache(launcher_dir: &Path) -> Result<aigl_python::Cache> {
    aigl_python::Cache::discover(&config::uv_cache_dir(launcher_dir))
}
