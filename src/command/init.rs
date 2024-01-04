use crate::util;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    util::path::create_nested_file(".git/refs/heads/main".to_string());
    let mut file = File::create(".git/HEAD").unwrap();
    file.write_all(b"ref: refs/heads/main\n").unwrap();
}
