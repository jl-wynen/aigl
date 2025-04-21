// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod project;

use anyhow::Result;
use project::Project;
use std::path::{Path, PathBuf};

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join("project")
}

async fn run() -> Result<()> {
    if project_root().exists() {
        tokio::fs::remove_dir_all(project_root()).await?;
    }

    let project = Project::init(project_root()).await?;
    let venv = aigl_python::VirtualEnvironment::create(
        project_root().join(".venv"),
        "3.13",
        project.python_cache(),
    )
    .await?;

    aigl_python::install(
        &[aigl_python::RequirementsSource::Package(
            "requests".to_owned(),
        )],
        false,
        project.python_cache(),
        &venv,
    )
    .await?;

    Ok(())
}

fn main() {
    // aigl_app::GameInstallApp::run();
    // Safety: This is single threaded code.
    unsafe {
        config::init_environment(&project_root());
    }

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
    result.unwrap();
}
