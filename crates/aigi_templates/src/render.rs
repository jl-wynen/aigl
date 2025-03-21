use super::filters;
use crate::fs::create_output_directory;
use anyhow::Result;
use minijinja::{Environment, Value};
use std::path::Path;

pub fn render_directory(src_path: &Path, dst_path: &Path, context: &Value) -> Result<()> {
    do_render_directory(src_path, dst_path, &mut make_environment(), context)
}

fn do_render_directory(
    src_path: &Path,
    dst_path: &Path,
    environment: &mut Environment,
    context: &Value,
) -> Result<()> {
    create_output_directory(dst_path)?;
    for entry in std::fs::read_dir(src_path)? {
        let entry = entry?;
        let src = entry.path();
        let dst = dst_path.join(entry.file_name());
        if src.is_dir() {
            do_render_directory(&src, &dst, environment, context)?;
        } else {
            do_render_file(&src, &dst, environment, context)?;
        }
    }
    Ok(())
}

fn do_render_file(
    src_path: &Path,
    dst_path: &Path,
    environment: &mut Environment,
    context: &Value,
) -> Result<()> {
    if src_path.extension().is_some_and(|e| e == "jinja") {
        let source = std::fs::read_to_string(src_path)?;
        let template = environment.template_from_str(&source)?;
        std::fs::write(dst_path, template.render(context)?)?;
    } else {
        std::fs::copy(src_path, dst_path)?;
    }
    Ok(())
}

fn make_environment() -> Environment<'static> {
    let mut env = Environment::new();
    env.add_filter("to_identifier", filters::to_identifier);
    env
}
