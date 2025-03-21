use aigi_template::render_directory;
use std::path::PathBuf;

fn input_path() -> PathBuf {
    PathBuf::from("tests").join("test_bot_template")
}

fn output_path() -> PathBuf {
    PathBuf::from("tests").join("rendered_bot")
}

#[test]
fn can_render_bot_template() {
    let dst_path = output_path();
    let context = aigi_template::context! {
        player_name => "testo",
        bot_name => "Test-O Bot",
    };
    render_directory(&input_path(), &dst_path, &context).unwrap();

    // std::fs::remove_dir_all(dst_path).unwrap()
}
