//
// use anyhow::Result;
// use git2::IndexAddOption;
// use std::fs::File;
// use std::io::Write;
// use std::path::{Path, PathBuf};
// fn add_and_commit_all(repo: &git2::Repository, message: &str) -> Result<()> {
//     let sig = repo.signature()?;
//     let parents = match repo.head() {
//         Ok(head) => vec![repo.find_commit(head.target().unwrap())?],
//         Err(_) => vec![],
//     };
//
//     // add
//     let mut index = repo.index()?;
//     index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
//     index.write()?;
//
//     // Commit
//     let new_tree_oid = index.write_tree()?;
//     let new_tree = repo.find_tree(new_tree_oid)?;
//     repo.commit(
//         Some("HEAD"),
//         &sig,
//         &sig,
//         message,
//         &new_tree,
//         // iter.collect to turn vec of values into vec of refs (there must be a better way)
//         &parents.iter().collect::<Vec<_>>(),
//     )?;
//     Ok(())
// }
//
// fn create_repo_and_commit_files() -> Result<()> {
//     let test_dir = PathBuf::from("test_dir");
//     if test_dir.exists() {
//         std::fs::remove_dir_all(&test_dir)?;
//     }
//     let repo = git2::Repository::init(&test_dir)?;
//
//     let file_path = test_dir.join("asd.txt");
//     File::create(&file_path)?.write_all(b"Test file asd")?;
//
//     add_and_commit_all(&repo, "Add asd.txt")?;
//
//     File::options()
//         .write(true)
//         .open(&file_path)?
//         .write_all(b"Updated asd.txt")?;
//     let file_path = test_dir.join("qwe.txt");
//     File::create(&file_path)?.write_all(b"Test file qwe")?;
//
//     add_and_commit_all(&repo, "Update")?;
//     Ok(())
// }
//
// fn main() -> Result<()> {
//     create_repo_and_commit_files()
// }

mod repo;
pub use repo::Repository;
