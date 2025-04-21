use super::config::{LAUNCHER_DIR_NAME, UV_CACHE_DIR_NAME};
use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

pub(crate) struct Project {
    path: PathBuf,
    python_cache: aigl_python::Cache,
}

impl Project {
    pub(crate) async fn init(path: PathBuf) -> Result<Self> {
        if path.exists() {
            bail!(
                "Cannot create project at {}: directory already exists",
                path.display()
            );
        }
        tokio::fs::create_dir_all(&path).await?;
        let launcher_dir = init_launcher_dir(&path).await?;
        let python_cache = init_python_cache(&launcher_dir)?;
        Ok(Self { path, python_cache })
    }

    pub(crate) fn python_cache(&self) -> &aigl_python::Cache {
        &self.python_cache
    }
}

async fn init_launcher_dir(project_root: &Path) -> Result<PathBuf> {
    let root = launcher_dir(project_root);
    tokio::fs::create_dir(&root).await?;
    cachedir::ensure_tag(&root)?;
    tokio::fs::write(root.join(".gitignore"), "*").await?;
    Ok(root)
}

fn init_python_cache(launcher_dir: &Path) -> Result<aigl_python::Cache> {
    aigl_python::Cache::init(&launcher_dir.join(UV_CACHE_DIR_NAME))
}

fn launcher_dir(project_root: &Path) -> PathBuf {
    project_root.join(LAUNCHER_DIR_NAME)
}
