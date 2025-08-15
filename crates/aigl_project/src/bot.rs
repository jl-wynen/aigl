use aigl_system::fs::copy_dir_recursive;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::Project;

pub type BotRenderArgs = HashMap<String, String>;

pub(crate) struct Bot {
    root: PathBuf,
}

impl Bot {
    pub(crate) async fn render_template(
        project: Arc<Mutex<Project>>,
        target: &Path,
        args: &BotRenderArgs,
    ) -> anyhow::Result<Self> {
        copy_bot_template(project.clone(), target).await?;
        let bot = Self {
            root: target.to_path_buf(),
        };
        bot.apply_args(args).await?;
        project
            .lock()
            .expect("Failed to get project lock")
            .cfg_mut()
            .bot_paths
            .push(target.to_path_buf());
        Ok(bot)
    }

    pub(crate) async fn apply_args(&self, args: &BotRenderArgs) -> anyhow::Result<()> {
        let config_path = self.config_file_path()?;
        let mut config = tokio::fs::read_to_string(&config_path)
            .await?
            .parse::<toml_edit::DocumentMut>()?;
        for (key, value) in args {
            config[key] = toml_edit::value(value);
        }
        tokio::fs::write(&config_path, config.to_string()).await?;

        let pyproject_path = self.pyproject_file_path();
        let mut pyproject = tokio::fs::read_to_string(&pyproject_path)
            .await?
            .parse::<toml_edit::DocumentMut>()?;
        pyproject["project"]["name"] = toml_edit::value(&args["name"]);
        tokio::fs::write(&pyproject_path, pyproject.to_string()).await?;

        Ok(())
    }

    fn package_src_path(&self) -> anyhow::Result<PathBuf> {
        let src_path = self.root.join("src");
        let mut entries = std::fs::read_dir(src_path)?;
        let entry = entries
            .next()
            .ok_or_else(|| anyhow::anyhow!("No entry found in src"))?;
        if entries.next().is_some() {
            return Err(anyhow::anyhow!("More than one entry found in src"));
        }
        Ok(entry?.path())
    }

    fn config_file_path(&self) -> anyhow::Result<PathBuf> {
        Ok(self.package_src_path()?.join("config.toml"))
    }

    fn pyproject_file_path(&self) -> PathBuf {
        self.root.join("pyproject.toml")
    }
}

async fn copy_bot_template(project: Arc<Mutex<Project>>, target: &Path) -> anyhow::Result<()> {
    let src = {
        let project = project.lock().expect("Failed to get project lock");
        project.cfg().bot_template_path.clone()
    };
    copy_dir_recursive(&src, target).await
}
