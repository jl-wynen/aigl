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

pub mod project {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ProjectConfig {
        pub game_config: game::GameConfig,
        pub venv_locations: HashMap<String, PathBuf>,
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
}

pub mod game {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct GameConfig {
        pub name: String,
        pub game: Game,
        pub bot: Bot,
        pub players: Players,
        pub python: Python,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Game {
        url: String,
        base_config_in_repo: PathBuf,
        launch_args: Vec<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Bot {
        template_url: String,
        #[serde(default)]
        template_args: HashMap<String, BotTemplateArg>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct BotTemplateArg {
        var: String,
        display: String,
        #[serde(default, rename = "type")]
        ty: BotTemplateArgType,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub enum BotTemplateArgType {
        #[default]
        #[serde(rename = "string")]
        String,
        #[serde(rename = "color", alias = "colour")]
        Color,
        #[serde(rename = "path")]
        Path,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(tag = "mode")]
    pub enum Players {
        #[serde(rename = "free-for-all")]
        FFA {
            n_min: usize,
            #[serde(default)]
            n_max: Option<usize>,
        },
        #[serde(rename = "teams")]
        Teams {
            n_teams_min: usize,
            #[serde(default)]
            n_teams_max: Option<usize>,
            n_bots_per_team_min: usize,
            #[serde(default)]
            n_bots_per_team_max: Option<usize>,
            #[serde(default)]
            team_names: Vec<String>,
            #[serde(default)]
            team_colors: Vec<String>,
        },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Python {
        pub version: String,
        #[serde(default)]
        pub venv: VenvKind,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub enum VenvKind {
        #[default]
        #[serde(rename = "single")]
        Single,
        #[serde(rename = "per-bot")]
        PerBot,
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
}
