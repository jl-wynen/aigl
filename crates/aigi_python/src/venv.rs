use crate::cache::Cache;
use crate::settings::uv_network_settings;
use anyhow::Result;
use std::path::{Path, PathBuf};
use uv::{commands, printer::Printer};
use uv_configuration::{Concurrency, IndexStrategy, KeyringProviderType, PreviewMode};
use uv_distribution_types::{DependencyMetadata, IndexLocations};
use uv_install_wheel::LinkMode;
use uv_python::{PythonDownloads, PythonPreference};
use uv_settings::PythonInstallMirrors;
use uv_virtualenv::Prompt;

pub async fn venv(path: &Path, python_request: &str, cache: &Cache) -> Result<()> {
    let _ = commands::venv(
        path,
        Some(path.to_owned()),
        Some(python_request),
        PythonInstallMirrors::default(),
        PythonPreference::OnlyManaged,
        PythonDownloads::Automatic,
        LinkMode::default(),
        &IndexLocations::default(),
        IndexStrategy::default(),
        DependencyMetadata::default(),
        KeyringProviderType::default(),
        &uv_network_settings(),
        Prompt::None,
        false,
        false,
        false,
        None,
        Concurrency::default(),
        true, // install independently of any projects
        true,
        cache.underlying(),
        Printer::Default, // TODO probably need more silent
        false,
        PreviewMode::Disabled,
    )
    .await?;
    Ok(())
}
