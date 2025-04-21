use crate::cache::Cache;
use crate::settings::uv_network_settings;
use crate::venv::VirtualEnvironment;
use anyhow::Result;
use std::path::PathBuf;
use uv::{
    commands::pip::{install::pip_install, operations::Modifications},
    printer::Printer,
};
use uv_configuration::{
    BuildOptions, Concurrency, ConfigSettings, DryRun, ExtrasSpecification, IndexStrategy,
    KeyringProviderType, PreviewMode, Reinstall, SourceStrategy, Upgrade,
};
use uv_distribution_types::{DependencyMetadata, IndexLocations};
use uv_install_wheel::LinkMode;
use uv_python::PythonPreference;
use uv_resolver::{DependencyMode, PrereleaseMode, ResolutionMode};

#[derive(Debug, Clone)]
pub enum RequirementsSource {
    Package(String),
    Editable(String),
    RequirementsTxt(PathBuf),
}

impl RequirementsSource {
    fn to_uv(&self) -> Result<uv_requirements::RequirementsSource> {
        use uv_requirements::RequirementsSource as uvr;
        match self {
            RequirementsSource::Package(package) => Ok(uvr::from_package_argument(package)?),
            RequirementsSource::Editable(editable) => Ok(uvr::from_editable(editable)?),
            RequirementsSource::RequirementsTxt(path) => {
                Ok(uvr::from_requirements_file(path.clone()))
            }
        }
    }
}

pub async fn install(
    requirements: &[RequirementsSource],
    compile: bool,
    cache: &Cache,
    environment: &VirtualEnvironment,
) -> Result<()> {
    let requirements = convert_requirements_to_uv(requirements)?;
    pip_install(
        &requirements,
        &[],
        &[],
        &[],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        &ExtrasSpecification::None,
        Default::default(),
        ResolutionMode::default(),
        PrereleaseMode::default(),
        DependencyMode::Transitive,
        Upgrade::default(),
        IndexLocations::default(),
        IndexStrategy::default(),
        None,
        DependencyMetadata::default(),
        KeyringProviderType::default(),
        &uv_network_settings(),
        Reinstall::default(),
        LinkMode::default(),
        compile,
        None,
        true,
        &ConfigSettings::default(),
        false,
        Vec::new(),
        BuildOptions::default(),
        Modifications::Sufficient, // Use `pip install` semantics.
        None,
        None,
        true,
        None,
        SourceStrategy::Disabled, // Prevent overriding sources.
        None,
        false,
        false,
        None,
        None,
        PythonPreference::OnlyManaged,
        Concurrency::default(),
        cache.underlying().clone(),
        DryRun::default(),
        Printer::Default, // TODO probably need more silent
        PreviewMode::Disabled,
        Some(environment.python_environment()),
    )
    .await?;
    Ok(())
}

fn convert_requirements_to_uv(
    requirements: &[RequirementsSource],
) -> Result<Vec<uv_requirements::RequirementsSource>> {
    let mut res = Vec::with_capacity(requirements.len());
    for req in requirements {
        res.push(req.to_uv()?);
    }
    Ok(res)
}
