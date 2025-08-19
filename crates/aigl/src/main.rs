// Hide the console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join("project")
}

fn main() {
    // Safety: This is single threaded code.
    unsafe {
        aigl_project::config::init_environment(&project_root());
    }

    aigl_app::GameInstallApp::run();
}
