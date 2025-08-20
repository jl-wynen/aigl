use aigl_git::Repository;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_path() -> PathBuf {
    PathBuf::from("tests").join("test_git_repo")
}

fn seed_directory(path: &Path) {
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }

    fs::create_dir(path).unwrap();
    fs::write(path.join("file1.txt"), "Test file 1").unwrap();
    let sub_dir = path.join("sub_dir");
    fs::create_dir(&sub_dir).unwrap();
    fs::write(sub_dir.join("file2"), "Second file").unwrap();
}

fn assert_repo_is_clean(path: &Path) {
    let status = Command::new("git")
        .args(["status"])
        .current_dir(path)
        .output()
        .unwrap();
    assert!(status.status.success());
    let stdout = String::from_utf8(status.stdout).unwrap();
    panic!("{}", &stdout);
    assert!(stdout.contains("main"));
    assert!(stdout.contains("nothing to commit"));
    assert!(stdout.contains("working tree clean"));
}

fn assert_git_history(path: &Path, expected_messages: &[&str]) {
    let status = Command::new("git")
        .args(["log", r#"--format=format:"%s""#])
        .current_dir(path)
        .output()
        .unwrap();
    assert!(status.status.success());
    let stdout = String::from_utf8(status.stdout).unwrap();
    for (line, &expected) in stdout.lines().zip(expected_messages) {
        assert_eq!(line, format!("\"{expected}\""));
    }
}

#[test]
fn can_init_and_commit_to_repository() {
    let path = repo_path();
    seed_directory(&path);

    let repo = Repository::init(&path).unwrap();
    repo.commit_all("Init").unwrap();
    assert_repo_is_clean(&path);
    assert_git_history(&path, &["Init"]);

    fs::write(path.join("file3.md"), "# THIRD").unwrap();
    fs::write(path.join("sub_dir").join("file4.txt"), "4").unwrap();
    let sub2 = path.join("sub_2");
    fs::create_dir(&sub2).unwrap();
    fs::write(sub2.join("file5.txt"), "fem").unwrap();
    repo.commit_all("Add more files").unwrap();
    assert_repo_is_clean(&path);
    assert_git_history(&path, &["Add more files", "Init"]);

    fs::write(path.join("file1.txt"), "Updated file 1").unwrap();
    fs::write(sub2.join("file5.txt"), "Updated 5").unwrap();
    repo.commit_all("Update files").unwrap();
    assert_repo_is_clean(&path);
    assert_git_history(&path, &["Update files", "Add more files", "Init"]);
}
