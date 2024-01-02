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
    pub time_stamp: usize,
}

impl Sign {
    pub fn to_string(&self) -> String {
        format!("{} <{}> {}", self.name, self.email, self.time_stamp)
    }
}
