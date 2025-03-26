use anyhow::{Result, bail};
use std::path::PathBuf;

pub struct Repository {
    repo: git2::Repository,
}

impl Repository {
    pub fn clone(url: &str, target: PathBuf, shallow: bool) -> Result<Self> {
        if target.exists() {
            bail!("Output directory {} already exists", target.display());
        }
        let git_config = git2::Config::open_default()?;

        let mut callbacks = git2::RemoteCallbacks::new();
        let mut cred_handler = git2_credentials::CredentialHandler::new(git_config);
        callbacks.credentials(move |url, username_from_url, _allowed_types| {
            cred_handler.try_next_credential(url, username_from_url, _allowed_types)
        });

        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        if shallow {
            fetch_options.depth(1);
        }

        let repo = git2::build::RepoBuilder::new()
            .fetch_options(fetch_options)
            .clone(url, &target)?;
        Ok(Self { repo })
    }
}
