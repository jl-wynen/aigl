use anyhow::Result;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

use aigl_project::{BotArg, Project, config::game::GameConfig};

#[derive(Debug, Default)]
pub(crate) struct InstallThreadData {
    pub(crate) error: Option<String>,
}

type Data = Arc<RwLock<InstallThreadData>>;

pub(crate) fn install(
    data: Data,
    target_path: PathBuf,
    config: GameConfig,
    player_bot_id: String,
    player_bot_name: String,
    player_bot_args: Vec<BotArg>,
) -> Option<()> {
    let runtime = start_tokio_runtime(&data)?;

    let result = runtime.block_on(Box::pin(async_install(
        target_path,
        config,
        player_bot_id,
        player_bot_name,
        player_bot_args,
    )));
    // Avoid waiting for pending tasks to complete.
    //
    // The resolver may have kicked off HTTP requests during resolution that
    // turned out to be unnecessary. Waiting for those to complete can cause
    // the CLI to hang before exiting.
    runtime.shutdown_background();

    if let Err(err) = result {
        show_error(&data, format!("Failed to install: {err}"));
    }
    Some(())
}

async fn async_install(
    target_path: PathBuf,
    config: GameConfig,
    player_bot_id: String,
    player_bot_name: String,
    player_bot_args: Vec<BotArg>,
) -> Result<Arc<Mutex<Project>>> {
    Project::init(
        target_path,
        config,
        player_bot_id,
        player_bot_name,
        player_bot_args,
    )
    .await
}

fn start_tokio_runtime(data: &Data) -> Option<tokio::runtime::Runtime> {
    match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => Some(runtime),
        Err(_) => {
            show_error(data, "Failed to create tokio runtime".to_string());
            None
        }
    }
}

fn show_error(data: &Data, error: String) {
    let start = Instant::now();
    while start.elapsed() < std::time::Duration::from_secs(1) {
        if let Ok(mut data) = data.write() {
            data.error = Some(error);
            break;
        }
    }
}
