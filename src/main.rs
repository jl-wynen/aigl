#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod game_config;
mod install_app;
mod theme;

fn main() {
    install_app::GameInstallApp::run();
}
