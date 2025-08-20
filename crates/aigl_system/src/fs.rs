use anyhow::{Context, bail};
use std::fs;
use std::path::{Path, PathBuf};

pub async fn create_output_directory(path: &Path) -> anyhow::Result<()> {
    path_available_as_output_directory(path)?;
    tokio::fs::create_dir_all(path)
        .await
        .context("When creating output directory")
}

pub fn path_available_as_output_directory(path: &Path) -> anyhow::Result<()> {
    if path.exists() {
        if !path.is_dir() {
            bail!(
                "Output path exists and is not a directory: {}",
                path.display()
            );
        } else if !directory_is_empty(path) {
            bail!("Output directory is not empty: {}", path.display());
        }
    }
    Ok(())
}

pub fn directory_is_empty(path: &Path) -> bool {
    let Ok(mut children) = fs::read_dir(path) else {
        return false;
    };
    children.next().is_none()
}

pub async fn copy_dir_recursive(
    src: impl Into<PathBuf>,
    dst: impl Into<PathBuf>,
) -> anyhow::Result<()> {
    let src = src.into();
    let dst = dst.into();
    tokio::task::spawn_blocking(move || copy_dir_recursive_blocking(&src, &dst)).await?
}

fn copy_dir_recursive_blocking(src: &Path, dst: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_recursive_blocking(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
