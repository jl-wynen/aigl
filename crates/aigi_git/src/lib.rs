//
// use anyhow::Result;
// use git2::IndexAddOption;
// use std::fs::File;
// use std::io::Write;
// use std::path::{Path, PathBuf};
//
// // fn clone_repo() {
// //     let url = "git@github.com:jl-wynen/laikr.git";
// //     let target = PathBuf::from("/home/jl/Prog/experiments/git-rs/laikr");
// //     if target.exists() {
// //         std::fs::remove_dir_all(&target).unwrap();
// //     }
// //
// //     let git_config = git2::Config::open_default().expect("Failed to open git config");
// //
// //     // This requires a working auth method.
// //     // It will not work in RustRover but does work in a terminal when my yubikey is plugged in
// //     let mut callbacks = git2::RemoteCallbacks::new();
// //     let mut cred_handler = git2_credentials::CredentialHandler::new(git_config);
// //     callbacks.credentials(move |_url, username_from_url, _allowed_types| {
// //         dbg!(_url, username_from_url, _allowed_types);
// //         // let cred = git2::Cred::ssh_key_from_agent(username_from_url.unwrap());
// //         let cred = cred_handler.try_next_credential(url, username_from_url, _allowed_types);
// //         dbg!(cred.is_ok());
// //         cred
// //     });
// //
// //     let mut fo = git2::FetchOptions::new();
// //     fo.remote_callbacks(callbacks);
// //
// //     let mut builder = git2::build::RepoBuilder::new();
// //     builder.fetch_options(fo);
// //
// //     let repo = builder.clone(url, &target).unwrap();
// // }
//
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
