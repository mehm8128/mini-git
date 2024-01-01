use std::fs;
use std::io;

pub fn find_git_root(path: String) -> Result<String, io::Error> {
    let files = fs::read_dir(path).unwrap();
    for file in files {
        let file = file.unwrap();
        if file.file_type().unwrap().is_dir() && file.file_name() == ".git" {
            return Ok(file.path().to_str().unwrap().to_string());
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Not found .git directory",
    ))
}
