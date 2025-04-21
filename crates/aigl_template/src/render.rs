use super::filters;
use aigl_system::fs::create_output_directory;
use anyhow::{Context, Result};
use minijinja::{Environment, Value};
use std::path::{Path, PathBuf};

pub async fn render_directory(src_path: &Path, dst_path: &Path, context: &Value) -> Result<()> {
    create_output_directory(dst_path).await?;
    let src = src_path.to_owned();
    let dst = dst_path.to_owned();
    let context = context.clone();
    tokio::task::spawn_blocking(move || {
        do_render_directory(src, dst, &make_environment(), &context)
    })
    .await?
}

// Use a blocking function here because most files are small and spawning separate
// tasks for all those files causes too much overhead.
fn do_render_directory(
    src_path: PathBuf,
    dst_path: PathBuf,
    environment: &Environment,
    context: &Value,
) -> Result<()> {
    for entry in std::fs::read_dir(src_path)? {
        let entry = entry?;
        let src = entry.path();
        if is_excluded(&src) {
            continue;
        }
        let dst = dst_path.join(render_path(
            entry
                .file_name()
                .into_string()
                .expect("Path name is not UTF-8"),
            environment,
            context,
        )?);
        if src.is_dir() {
            std::fs::create_dir(&dst)?;
            do_render_directory(src, dst, environment, context)?;
        } else {
            do_render_file(&src, &dst, environment, context)?;
        }
    }
    Ok(())
}

fn do_render_file(
    src_path: &Path,
    dst_path: &Path,
    environment: &Environment,
    context: &Value,
) -> Result<()> {
    if src_path.extension().is_some_and(|e| e == "jinja") {
        let source = std::fs::read_to_string(src_path)?;
        let template = environment.template_from_str(&source)?;
        std::fs::write(dst_path.with_extension(""), template.render(context)?)?;
    } else {
        std::fs::copy(src_path, dst_path)?;
    }
    Ok(())
}

fn render_path(path: String, environment: &Environment, context: &Value) -> Result<PathBuf> {
    let template = environment.template_from_str(&path)?;
    Ok(PathBuf::from(template.render(context)?))
}

fn make_environment() -> Environment<'static> {
    let mut env = Environment::new();
    env.add_filter("to_identifier", filters::to_identifier);
    env
}

fn is_excluded(path: &Path) -> bool {
    path.file_name().is_some_and(|name| name == ".git")
}
