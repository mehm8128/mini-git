use crate::object::commit::{Commit, Sign};
use crate::util;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs::File, io::Write};

pub fn commit(message: String) {
    decode_index_file();
    let tree_hash = "".to_string();
    let commit_hash = generate_commit_object(tree_hash, message);
    update_head(commit_hash);
}

fn decode_index_file() {
    let mut file = File::open(".git/index").unwrap();
    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap();
    println!("{:?}", content);
}

fn generate_commit_object(tree_hash: String, message: String) -> String {
    let parent = util::path::get_head_commit();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut commit = Commit {
        hash: "".to_string(),
        size: 0,
        tree: tree_hash,
        parents: match parent {
            Some(parent) => vec![parent],
            None => vec![],
        },
        author: Sign {
            name: "mehm8128".to_string(),
            email: "mehm8128@example.com".to_string(),
            time_stamp: now,
        },
        commiter: Sign {
            name: "mehm8128".to_string(),
            email: "mehm8128@example.com".to_string(),
            time_stamp: now,
        },
        message,
    };

    let mut content: Vec<u8> = Vec::new();
    content.extend(format!("tree {}\n", commit.tree).as_bytes());
    for parent in commit.parents {
        content.extend(format!("parent {}\n", parent).as_bytes());
    }
    content.extend(format!("author {}\n", commit.author.to_string()).as_bytes());
    content.extend(format!("committer {}\n", commit.commiter.to_string()).as_bytes());
    content.extend(format!("\n{}\n", commit.message).as_bytes());

    commit.size = content.len();
    let header = format!("commit {}\0", commit.size);
    let content = format!("{}{}", header, String::from_utf8(content).unwrap());
    let commit_hash = util::compress::hash(&content);
    commit.hash = commit_hash;

    let file_directory = format!(".git/objects/{}", &commit.hash[0..2]);
    let file_path = format!("{}/{}", file_directory, &commit.hash[2..]);
    std::fs::create_dir(file_directory).unwrap();
    let mut file = File::create(file_path).unwrap();

    // zlib圧縮
    let compressed_contents = util::compress::zlib_compress(content);
    file.write_all(&compressed_contents).unwrap();

    commit.hash
}

fn update_head(commit_hash: String) {
    let head_ref = util::path::get_head_ref();
    let mut file = File::create(head_ref).unwrap();
    file.write_all(commit_hash.as_bytes()).unwrap();
}
