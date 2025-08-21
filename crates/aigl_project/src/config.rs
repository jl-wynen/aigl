use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const LAUNCHER_DIR_NAME: &str = ".aigl";
pub const PYTHON_DIR_NAME: &str = "python";
pub const UV_CACHE_DIR_NAME: &str = "uv_cache";
pub const BOT_TEMPLATES_DIR_NAME: &str = "bot_templates";

pub const PROJECT_CONFIG_FILE_NAME: &str = "project.json";

pub(crate) fn launcher_dir(project_root: &Path) -> PathBuf {
    project_root.join(LAUNCHER_DIR_NAME)
}

pub(crate) fn python_dir(launcher_root: &Path) -> PathBuf {
    launcher_root.join(PYTHON_DIR_NAME)
}

pub(crate) fn uv_cache_dir(launcher_root: &Path) -> PathBuf {
    launcher_root.join(UV_CACHE_DIR_NAME)
}

pub(crate) fn bot_templates_dir(project_root: &Path) -> PathBuf {
    launcher_dir(project_root).join(BOT_TEMPLATES_DIR_NAME)
}

pub(crate) fn project_config_file(project_root: &Path) -> PathBuf {
    launcher_dir(project_root).join(PROJECT_CONFIG_FILE_NAME)
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
        pub game_path: PathBuf,
        // The first bot is the player
        pub bot_paths: Vec<PathBuf>,
        pub bot_template_path: PathBuf,
        pub venv_paths: HashMap<String, PathBuf>,
    }

    impl ProjectConfig {
        pub async fn load_json(path: &Path) -> Result<Self> {
            Ok(serde_json::from_str(
                &tokio::fs::read_to_string(path).await?,
            )?)
        }

        pub async fn save_json(&self, path: &Path) -> Result<()> {
            tokio::fs::write(path, serde_json::to_string(self)?).await?;
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
        pub url: String,
        pub base_config_in_repo: PathBuf,
        pub launch_args: Vec<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Bot {
        pub template_url: String,
        #[serde(default)]
        pub template_args: HashMap<String, BotTemplateArg>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct BotTemplateArg {
        pub var: String,
        pub display: String,
        #[serde(default, rename = "type")]
        pub ty: BotTemplateArgType,
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
            #[serde(default)]
            n_initial: Option<usize>,
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
        pub async fn load_toml(path: &Path) -> Result<Self> {
            Self::load_toml_str(&tokio::fs::read_to_string(path).await?)
        }

        pub fn load_toml_str(content: &str) -> Result<Self> {
            Ok(toml_edit::de::from_str(content)?)
        }

        pub async fn save_toml(&self, path: &Path) -> Result<()> {
            tokio::fs::write(path, toml_edit::ser::to_string(self)?).await?;
            Ok(())
        }
    }
}
