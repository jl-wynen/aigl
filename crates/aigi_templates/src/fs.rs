use anyhow::{Context, bail};
use std::path::Path;

pub(crate) fn create_output_directory(path: &Path) -> anyhow::Result<()> {
    if path.exists() && !directory_is_empty(path) {
        bail!("Output directory is not empty: {}", path.display());
    }
    std::fs::create_dir_all(path).context("When creating output directory")
}

pub(crate) fn directory_is_empty(path: &Path) -> bool {
    let Ok(mut children) = path.read_dir() else {
        return false;
    };
    children.next().is_none()
}
