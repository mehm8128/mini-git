use std::fmt;

use chrono::{self, DateTime, Local};

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
    pub time_stamp: DateTime<Local>,
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            name,
            email,
            time_stamp,
        } = self;
        let timestamp = time_stamp.timestamp();
        let timezone = self.time_stamp.format("%z").to_string();
        write!(f, "{name} <{email}> {timestamp} {timezone}")
    }
}
