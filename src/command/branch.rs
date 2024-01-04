use crate::util;
use std::{fs, io::Write};

pub fn branch(branch_name: String) {
    let head_commit_hash = match util::path::get_head_commit_hash() {
        Some(hash) => hash,
        None => "".to_string(),
    };
    let mut file = fs::File::create(format!(".git/refs/heads/{}", branch_name)).unwrap();
    file.write_all(head_commit_hash.as_bytes()).unwrap();
    let mut file = fs::File::create(".git/HEAD").unwrap();
    let content = format!("ref: refs/heads/{}", branch_name);
    file.write_all(content.as_bytes()).unwrap();
}

pub fn delete_branch(branch_name: String) {
    let head_ref = util::path::get_head_ref();
    if head_ref == format!(".git/refs/heads/{}", branch_name) {
        println!("Cannot delete current branch");
        return;
    }
    fs::remove_file(format!(".git/refs/heads/{}", branch_name)).unwrap();
}

pub fn checkout(branch_name: String) {
    let mut file = fs::File::create(".git/HEAD").unwrap();
    let content = format!("ref: refs/heads/{}", branch_name);
    file.write_all(content.as_bytes()).unwrap();
}
