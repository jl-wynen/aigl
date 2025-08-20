use crate::cache::Cache;
use crate::settings::uv_network_settings;
use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use uv::{commands, printer::Printer};
use uv_configuration::{Concurrency, IndexStrategy, KeyringProviderType, Preview};
use uv_distribution_types::{DependencyMetadata, IndexLocations};
use uv_install_wheel::LinkMode;
use uv_python::{
    ImplementationName, PythonDownloads, PythonEnvironment, PythonPreference, PythonRequest,
    VersionRequest,
};
use uv_resolver::{ExcludeNewer, ExcludeNewerPackage};
use uv_settings::PythonInstallMirrors;
use uv_virtualenv::{OnExisting, Prompt};

pub struct VirtualEnvironment {
    python_environment: PythonEnvironment,
}

impl VirtualEnvironment {
    pub async fn create(root: PathBuf, python_request: &str, cache: &Cache) -> Result<Self> {
        let python_request = PythonRequest::ImplementationVersion(
            ImplementationName::CPython,
            VersionRequest::from_str(python_request)?,
        );
        let _ = commands::venv(
            &root,
            Some(root.clone()),
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
            true,
            OnExisting::Fail,
            ExcludeNewer::new(None, ExcludeNewerPackage::default()),
            Concurrency::default(),
            true, // install independently of any projects
            true,
            cache.underlying(),
            Printer::Default, // TODO probably need more silent
            false,
            Preview::default(),
        )
        .await?;
        Self::new(root, cache)
    }

    pub fn open(root: PathBuf, cache: &Cache) -> Result<Self> {
        Self::new(root, cache)
    }

    fn new(root: PathBuf, cache: &Cache) -> Result<Self> {
        let python_environment = PythonEnvironment::from_root(&root, cache.underlying())?;
        Ok(Self { python_environment })
    }

    pub fn python_executable(&self) -> PathBuf {
        self.python_environment()
            .into_interpreter()
            .real_executable()
            .to_owned()
    }

    pub fn prepare_python_command(&self) -> Command {
        Command::new(self.python_executable())
    }

    pub(crate) fn python_environment(&self) -> PythonEnvironment {
        self.python_environment.clone()
    }
}
