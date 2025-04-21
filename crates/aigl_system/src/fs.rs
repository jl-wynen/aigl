use anyhow::{Context, bail};
use std::path::Path;

pub async fn create_output_directory(path: &Path) -> anyhow::Result<()> {
    if path.exists() {
        if !path.is_dir() {
            bail!(
                "Output path exists and is not a directory: {}",
                path.display()
            );
        } else if !directory_is_empty(path).await {
            bail!("Output directory is not empty: {}", path.display());
        }
    }
    tokio::fs::create_dir_all(path)
        .await
        .context("When creating output directory")
}

pub async fn directory_is_empty(path: &Path) -> bool {
    let Ok(mut children) = tokio::fs::read_dir(path).await else {
        return false;
    };
    children.next_entry().await.is_err()
}
