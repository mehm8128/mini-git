use std::fs;
use std::io;
use std::io::ErrorKind;

pub fn find_git_root() -> Result<String, io::Error> {
    let files = fs::read_dir(".").unwrap();
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

pub fn get_head_ref() -> String {
    let head = fs::read_to_string(".git/HEAD").unwrap();
    let head: Vec<&str> = head.split(" ").collect();

    format!(".git/{}", head[1].to_string().trim_end().to_string())
}

pub fn get_head_commit() -> Option<String> {
    let head_ref = get_head_ref();
    let head_commit = fs::read_to_string(head_ref);
    match head_commit {
        Ok(head_commit) if head_commit != "" => Some(head_commit),
        Ok(_) => None,
        Err(ref e) if e.kind() == ErrorKind::NotFound => None,
        Err(e) => panic!("{}", e),
    }
}
