// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use std::path::{Path, PathBuf};

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join("project")
}

fn launcher_root() -> PathBuf {
    project_root().join(".aigi")
}

async fn init_launcher_dir() -> Result<PathBuf> {
    let root = launcher_root();
    tokio::fs::create_dir_all(&root).await?;
    cachedir::ensure_tag(&root)?;
    Ok(root)
}

async fn run() {
    if launcher_root().exists() {
        tokio::fs::remove_dir_all(launcher_root()).await.unwrap();
    }
    init_launcher_dir().await.unwrap();
    let cache = aigi_python::Cache::init(&launcher_root()).unwrap();
    aigi_python::venv(&project_root().join(".venv"), "3.13", &cache)
        .await
        .unwrap();
}

fn main() {
    // aigi_app::GameInstallApp::run();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

    let result = runtime.block_on(Box::pin(run()));
    // Avoid waiting for pending tasks to complete.
    //
    // The resolver may have kicked off HTTP requests during resolution that
    // turned out to be unnecessary. Waiting for those to complete can cause
    // the CLI to hang before exiting.
    runtime.shutdown_background();
    // result.unwrap();
}
