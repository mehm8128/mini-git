use std::fs;
use std::fs::File;
use std::io::Write;

pub fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir_all(".git/refs/heads").unwrap();
    let mut file = File::create(".git/HEAD").unwrap();
    file.write_all(b"ref: refs/heads/master\n").unwrap();
}
