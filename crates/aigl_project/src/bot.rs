use async_lock::Mutex;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::Project;
use crate::config::game::{BotTemplateArg, BotTemplateArgType};
use aigl_system::fs::copy_dir_recursive;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BotArgValue {
    String(String),
    Color([u8; 3]),
    Path(String),
}

impl BotArgValue {
    pub fn serialize_value(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Color(c) => format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2]),
            Self::Path(p) => p.clone(),
        }
    }

    pub fn color_from_string(value: &str) -> Self {
        let mut color = [0; 3];
        let mut i = 0;
        let mut str_buf = [0u8; 4];
        value.trim_start_matches('#').chars().for_each(|c| {
            if i >= 6 {
                return; // ignore anything beyond the first 3 hex bytes
            }
            let s = c.encode_utf8(&mut str_buf);
            // Silently handle invalid hex characters to keep error handling simple.
            let byte = u8::from_str_radix(s, 16).unwrap_or(0);
            color[i / 2] += byte * (16 * (i % 2) as u8);
            i += 1;
        });
        Self::Color(color)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotArg {
    pub var: String,
    pub display: String,
    pub value: BotArgValue,
}

impl BotArg {
    pub fn default_from_template_arg(template_arg: BotTemplateArg) -> Self {
        let value = match template_arg.ty {
            BotTemplateArgType::String => BotArgValue::String(String::new()),
            BotTemplateArgType::Color => BotArgValue::Color([0, 0, 0]),
            BotTemplateArgType::Path => BotArgValue::Path(String::new()),
        };
        Self {
            var: template_arg.var,
            display: template_arg.display,
            value,
        }
    }
}

pub(crate) struct Bot {
    root: PathBuf,
    id: String,
    name: String,
    args: Vec<BotArg>,
}

impl Bot {
    pub(crate) async fn render_template(
        project: Arc<Mutex<Project>>,
        target: &Path,
        id: String,
        name: String,
        args: Vec<BotArg>,
    ) -> anyhow::Result<Self> {
        copy_bot_template(project.clone(), target).await?;
        let bot = Self {
            root: target.to_path_buf(),
            id,
            name,
            args,
        };
        bot.apply_args(&bot.id, &bot.name, &bot.args).await?;
        project
            .lock()
            .await
            .cfg_mut()
            .bot_paths
            .push(target.to_path_buf());
        Ok(bot)
    }

    pub(crate) async fn apply_args(
        &self,
        id: &str,
        name: &str,
        args: &[BotArg],
    ) -> anyhow::Result<()> {
        self.move_package_src(id)?;

        let config_path = self.config_file_path()?;
        let mut config = tokio::fs::read_to_string(&config_path)
            .await?
            .parse::<toml_edit::DocumentMut>()?;
        config["id"] = toml_edit::value(id);
        config["name"] = toml_edit::value(name);
        for arg in args {
            config[&arg.var] = toml_edit::value(arg.value.serialize_value());
        }
        tokio::fs::write(&config_path, config.to_string()).await?;

        let pyproject_path = self.pyproject_file_path();
        let mut pyproject = tokio::fs::read_to_string(&pyproject_path)
            .await?
            .parse::<toml_edit::DocumentMut>()?;
        pyproject["project"]["name"] = toml_edit::value(id);
        pyproject["tool"]["ruff"]["lint"]["isort"]["known-first-party"] = toml_edit::value(id);
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

    fn move_package_src(&self, new_name: &str) -> anyhow::Result<()> {
        let old = self.package_src_path()?;
        let new = old.with_file_name(new_name);
        std::fs::rename(old, new)?;
        Ok(())
    }
}

async fn copy_bot_template(project: Arc<Mutex<Project>>, target: &Path) -> anyhow::Result<()> {
    let src = {
        let project = project.lock().await;
        project.cfg().bot_template_path.clone()
    };
    copy_dir_recursive(&src, target).await
}
