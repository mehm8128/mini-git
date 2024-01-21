use crate::util;
use std::{fs, io::Write};

pub fn branch(branch_name: String) -> std::io::Result<()> {
    let head_commit_hash = match util::path::get_head_commit_hash() {
        Some(hash) => hash,
        None => "".to_string(),
    };
    let mut file = util::path::create_nested_file(format!(".git/refs/heads/{}", branch_name));
    file.write_all(head_commit_hash.as_bytes())?;
    let mut file = fs::File::create(".git/HEAD")?;
    let content = format!("ref: refs/heads/{}", branch_name);
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn delete_branch(branch_name: String) -> std::io::Result<()> {
    let head_ref = util::path::get_head_ref();
    if head_ref == format!(".git/refs/heads/{}", branch_name) {
        // Errを返した方がいいかも
        println!("Cannot delete current branch");
        return Ok(());
    }
    fs::remove_file(format!(".git/refs/heads/{}", branch_name))?;
    Ok(())
}

pub fn checkout(branch_name: String) -> std::io::Result<()> {
    let mut file = fs::File::create(".git/HEAD")?;
    let content = format!("ref: refs/heads/{}", branch_name);
    file.write_all(content.as_bytes())?;
    Ok(())
}
