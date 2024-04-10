use std::{fs, io::Write};

use crate::util;

pub fn create(branch_name: &str) -> std::io::Result<()> {
    let head_commit_hash = util::path::get_head_commit_hash().unwrap_or_default();
    let mut file = util::path::create_nested_file(format!(".git/refs/heads/{branch_name}"));
    file.write_all(head_commit_hash.as_bytes())?;
    let mut file = fs::File::create(".git/HEAD")?;
    let content = format!("ref: refs/heads/{branch_name}");
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn delete(branch_name: &str) -> std::io::Result<()> {
    let head_ref = util::path::get_head_ref();
    if head_ref == format!(".git/refs/heads/{branch_name}") {
        // Errを返した方がいいかも
        println!("Cannot delete current branch");
        return Ok(());
    }
    fs::remove_file(format!(".git/refs/heads/{branch_name}"))?;
    Ok(())
}

pub fn checkout(branch_name: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(".git/HEAD")?;
    let content = format!("ref: refs/heads/{branch_name}");
    file.write_all(content.as_bytes())?;
    Ok(())
}
