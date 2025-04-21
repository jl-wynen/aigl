use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const LAUNCHER_DIR_NAME: &str = ".aigl";
pub const PYTHON_DIR_NAME: &str = "python";
pub const UV_CACHE_DIR_NAME: &str = "uv_cache";

pub const PROJECT_CONFIG_FILE_NAME: &str = "project.toml";

pub(crate) fn launcher_dir(project_root: &Path) -> PathBuf {
    project_root.join(LAUNCHER_DIR_NAME)
}

pub(crate) fn python_dir(launcher_root: &Path) -> PathBuf {
    launcher_root.join(PYTHON_DIR_NAME)
}

pub(crate) fn uv_cache_dir(launcher_root: &Path) -> PathBuf {
    launcher_root.join(UV_CACHE_DIR_NAME)
}

pub(crate) fn project_config_file(launcher_root: &Path) -> PathBuf {
    launcher_root.join(PROJECT_CONFIG_FILE_NAME)
}

/// Initialise the environment for the given project directory.
///
/// # Safety
///
/// This function sets environment variables and is only safe when called
/// in single-threaded code.
pub unsafe fn init_environment(project_root: &Path) {
    let launcher_root = launcher_dir(project_root);
    unsafe {
        // Required to control where Python is installed.
        std::env::set_var("UV_PYTHON_INSTALL_DIR", python_dir(&launcher_root));
        // Should be unused, but setting it here for good measure.
        std::env::set_var("UV_CACHE_DIR", uv_cache_dir(&launcher_root));
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub game: GameConfig,
    pub venv_locations: HashMap<String, PathBuf>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub name: String,
    #[serde(default)]
    pub venv: GameVenvSpec,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum GameVenvSpec {
    #[default]
    #[serde(rename = "single")]
    Single,
}

impl ProjectConfig {
    pub fn load_toml(path: &Path) -> Result<Self> {
        Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
    }

    pub fn save_toml(&self, path: &Path) -> Result<()> {
        std::fs::write(path, toml::to_string(self)?)?;
        Ok(())
    }
}

impl GameConfig {
    pub fn load_toml(path: &Path) -> Result<Self> {
        Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
    }

    pub fn save_toml(&self, path: &Path) -> Result<()> {
        std::fs::write(path, toml::to_string(self)?)?;
        Ok(())
    }
}
