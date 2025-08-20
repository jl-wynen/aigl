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
    BuildOptions, Concurrency, DryRun, ExtrasSpecification, IndexStrategy, KeyringProviderType,
    Reinstall, SourceStrategy, Upgrade,
};
use uv_distribution_types::{
    ConfigSettings, DependencyMetadata, ExtraBuildVariables, IndexLocations, PackageConfigSettings,
};
use uv_install_wheel::LinkMode;
use uv_normalize::{GroupName, PipGroupName};
use uv_python::{PythonPreference, Target};
use uv_requirements::GroupsSpecification;
use uv_resolver::{
    DependencyMode, ExcludeNewer, ExcludeNewerPackage, PrereleaseMode, ResolutionMode,
};
use uv_workspace::pyproject::ExtraBuildDependencies;

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
            RequirementsSource::RequirementsTxt(path) => uvr::from_requirements_file(path.clone()),
        }
    }
}

pub async fn install(
    requirements: &[RequirementsSource],
    groups: impl IntoIterator<Item = (PathBuf, String)>,
    compile: bool,
    cache: &Cache,
    environment: &VirtualEnvironment,
) -> Result<()> {
    let requirements = convert_requirements_to_uv(requirements)?;
    let groups = GroupsSpecification {
        root: PathBuf::new(),
        groups: groups
            .into_iter()
            .map(|(path, name)| PipGroupName {
                path: Some(path),
                name: GroupName::from_owned(name).unwrap(),
            })
            .collect(),
    };

    pip_install(
        &requirements,
        &[],
        &[],
        &[],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        &ExtrasSpecification::default(),
        &groups,
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
        &PackageConfigSettings::default(),
        false,
        Vec::new(),
        &ExtraBuildDependencies::default(),
        &ExtraBuildVariables::default(),
        BuildOptions::default(),
        Modifications::Sufficient, // Use `pip install` semantics.
        None,
        None,
        true,
        ExcludeNewer::new(None, ExcludeNewerPackage::default()),
        SourceStrategy::Disabled, // Prevent overriding sources.
        Some(environment.python_executable().display().to_string()),
        false,
        false,
        None,
        None,
        PythonPreference::OnlyManaged,
        Concurrency::default(),
        cache.underlying().clone(),
        DryRun::default(),
        Printer::Default, // TODO probably need more silent
        Default::default(),
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
