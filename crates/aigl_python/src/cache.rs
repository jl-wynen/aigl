use anyhow::{Result, bail};
use std::path::Path;

const CACHE_DIR_NAME: &str = "uv_cache";

pub struct Cache {
    underlying: uv_cache::Cache,
}

impl Cache {
    pub fn init(launcher_root: &Path) -> Result<Self> {
        let underlying = uv_cache::Cache::from_path(launcher_root.join(CACHE_DIR_NAME));
        Ok(Cache {
            underlying: underlying.init()?,
        })
    }

    pub fn discover(launcher_root: &Path) -> Result<Self> {
        let cache_dir = launcher_root.join(CACHE_DIR_NAME);
        if !cache_dir.exists() {
            bail!("Cache directory does not exist: {}", cache_dir.display());
        }
        Ok(Self {
            underlying: uv_cache::Cache::from_path(cache_dir),
        })
    }

    pub fn underlying(&self) -> &uv_cache::Cache {
        &self.underlying
    }
}
