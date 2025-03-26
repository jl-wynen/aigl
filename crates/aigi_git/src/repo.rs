use anyhow::{Result, bail};
use std::path::Path;

pub struct Repository {
    repo: git2::Repository,
}

impl Repository {
    pub fn clone(url: &str, target: &Path, shallow: bool) -> Result<Self> {
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
            .clone(url, target)?;
        Ok(Self { repo })
    }

    pub fn init(path: &Path) -> Result<Self> {
        let repo = git2::Repository::init(path)?;
        Ok(Self { repo })
    }

    /// Commit all changes to the repository and advance HEAD.
    pub fn commit_all(&self, message: &str) -> Result<()> {
        let mut index = self.add_all()?;
        self.commit_to_head(message, &mut index)
    }

    /// Add all changes to the index.
    fn add_all(&self) -> Result<git2::Index> {
        let mut index = self.repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        Ok(index)
    }

    /// Write current index and advance HEAD.
    fn commit_to_head(&self, message: &str, index: &mut git2::Index) -> Result<()> {
        let signature = self.repo.signature()?;
        let parents = self.parents_from_head()?;
        let new_tree_oid = index.write_tree()?;
        let new_tree = self.repo.find_tree(new_tree_oid)?;
        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &new_tree,
            // iter.collect to turn vec of values into vec of refs (there must be a better way!)
            &parents.iter().collect::<Vec<_>>(),
        )?;
        Ok(())
    }

    /// Get parent commits for making a new commit based on current HEAD.
    fn parents_from_head(&self) -> Result<Vec<git2::Commit>> {
        Ok(match self.repo.head() {
            Ok(head) => vec![self.repo.find_commit(head.target().unwrap())?],
            Err(_) => vec![],
        })
    }
}
