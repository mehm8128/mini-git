use std::fmt;

pub struct Commit {
    pub hash: String,
    pub size: usize,
    pub tree: String,
    pub parents: Vec<String>,
    pub author: Sign,
    pub commiter: Sign,
    pub message: String,
}

pub struct Sign {
    pub name: String,
    pub email: String,
    pub time_stamp: u64,
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} <{}> {} +0900",
            self.name, self.email, self.time_stamp
        )
    }
}
