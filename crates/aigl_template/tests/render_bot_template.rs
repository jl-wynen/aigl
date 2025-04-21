use aigl_template::render_directory;
use std::path::PathBuf;

fn input_path() -> PathBuf {
    PathBuf::from("tests").join("test_bot_template")
}

fn output_path() -> PathBuf {
    PathBuf::from("tests").join("rendered_bot")
}

fn assert_in_file(expected_str: &str, segments: &[&str]) {
    let mut path = output_path();
    for segment in segments {
        path.push(segment);
    }
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains(expected_str));
}

#[tokio::test]
async fn can_render_bot_template() {
    let dst_path = output_path();
    if dst_path.exists() {
        std::fs::remove_dir_all(&dst_path).unwrap();
    }

    let context = aigl_template::context! {
        player_name => "test_o",
        bot_name => "Test-O Bot",
    };
    render_directory(&input_path(), &dst_path, &context)
        .await
        .unwrap();

    assert_in_file(r#"name = "test_o""#, &["pyproject.toml"]);
    assert_in_file(r#"class TestOBot:"#, &["src", "test_o", "__init__.py"]);
    assert_in_file(r#"*.egg-info"#, &[".gitignore"]);
}
