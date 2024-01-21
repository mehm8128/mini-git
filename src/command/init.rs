use crate::util;
use std::fs;
use std::fs::File;
use std::io::{Result, Write};

pub fn init() -> Result<()> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    util::path::create_nested_file(".git/refs/heads/main".to_string());
    let mut file = File::create(".git/HEAD")?;
    file.write_all(b"ref: refs/heads/main\n")?;
    Ok(())
}
