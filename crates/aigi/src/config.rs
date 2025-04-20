use std::path::{Path, PathBuf};

pub const LAUNCHER_DIR_NAME: &str = ".aigi";
pub const PYTHON_DIR_NAME: &str = "python";
pub const UV_CACHE_DIR_NAME: &str = "uv_cache";

pub fn launcher_dir(project_root: &Path) -> PathBuf {
    project_root.join(LAUNCHER_DIR_NAME)
}

pub fn python_dir(launcher_root: &Path) -> PathBuf {
    launcher_root.join(PYTHON_DIR_NAME)
}

pub fn uv_cache_dir(launcher_root: &Path) -> PathBuf {
    launcher_root.join(UV_CACHE_DIR_NAME)
}

/// Initialise the environment for the given project directory.
///
/// # Safety
///
/// This function sets environment variables and is only safe when called
/// in single-threaded code.
pub unsafe fn init_environment(project_root: &Path) {
    let launcher_root = launcher_dir(project_root);
    unsafe {
        // Required to control where Python is installed.
        std::env::set_var("UV_PYTHON_INSTALL_DIR", python_dir(&launcher_root));
        // Should be unused, but setting it here for good measure.
        std::env::set_var("UV_CACHE_DIR", uv_cache_dir(&launcher_root));
    }
}
