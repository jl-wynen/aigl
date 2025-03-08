// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;
mod game_config;
mod install_app;
mod theme;

fn main() {
    install_app::GameInstallApp::run();
}
