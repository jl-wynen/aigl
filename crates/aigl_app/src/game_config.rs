use anyhow::{Result, bail};

use aigl_project::config::game::GameConfig;

const BASE_URL: &str = "https://jl-wynen.github.io/aigl/resources/games";

pub(crate) fn fetch_game_config(game_code: &str) -> Result<GameConfig> {
    let url = format!("{BASE_URL}/{game_code}.toml");
    let config_toml = reqwest::blocking::get(url)?.text()?;
    if config_toml.starts_with("<!DOCTYPE html>") {
        bail!("Game '{game_code}' not found");
    }
    GameConfig::load_toml_str(&config_toml)
}
