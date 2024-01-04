use crate::util;
use hex;
use std::fs::{self, File};
use std::io::Write;
use std::os::linux::fs::MetadataExt;

struct IndexHeader {
    signature: [u8; 4],
    version: [u8; 4],
    entries: [u8; 4],
}

struct IndexEntry {
    ctime: [u8; 4],
    ctime_nsec: [u8; 4],
    mtime: [u8; 4],
    mtime_nsec: [u8; 4],
    dev: [u8; 4],
    ino: [u8; 4],
    mode: [u8; 4],
    uid: [u8; 4],
    gid: [u8; 4],
    file_size: [u8; 4],
    oid: String, // 20byte
    flags: [u8; 2],
    path: String,
}

fn travel_dir(file_name: &String, file_path_list: &mut Vec<String>, hash_list: &mut Vec<String>) {
    if fs::metadata(file_name).unwrap().is_dir() {
        // 再帰的にaddする
        for entry in fs::read_dir(file_name).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.starts_with("./.git") {
                continue;
            }
            if path.is_dir() {
                travel_dir(
                    &path.to_str().unwrap().to_string(),
                    file_path_list,
                    hash_list,
                );
                continue;
            }
            let file_name = path.to_str().unwrap().to_string();
            let hash = generate_blob_object(&file_name);
            file_path_list.push(file_name);
            hash_list.push(hash);
        }
    } else {
        let hash = generate_blob_object(file_name);
        file_path_list.push(file_name.clone());
        hash_list.push(hash);
    }
}

pub fn add(file_names: &[String]) {
    let mut hash_list = Vec::new();
    let mut file_path_list = Vec::new();
    for file_name in file_names {
        travel_dir(file_name, &mut file_path_list, &mut hash_list);
    }
    update_index(&file_path_list, hash_list);
}

fn generate_blob_object(file_name: &String) -> String {
    let contents = fs::read_to_string(file_name).unwrap();
    let file_length = contents.len();

    // データの準備
    let header = format!("blob {}\0", file_length);
    let hash = util::compress::hash(&format!("{}{}", header, contents).as_bytes());

    // ファイルの準備
    let file_directory = format!(".git/objects/{}", &hash[0..2]);
    let file_path = format!("{}/{}", file_directory, &hash[2..]);
    std::fs::create_dir_all(file_directory).unwrap();
    let mut file = File::create(file_path).unwrap();

    // zlib圧縮
    let contents_will_be_compressed = format!("{}{}", header, contents);
    let compressed_contents =
        util::compress::zlib_compress(&contents_will_be_compressed.as_bytes());

    // ファイルに書き込み
    file.write_all(&compressed_contents).unwrap();

    hash
}

fn update_index(file_names: &[String], hash_list: Vec<String>) {
    let mut content: Vec<u8> = Vec::new();

    // header
    let index_header = IndexHeader {
        signature: "DIRC".as_bytes().try_into().unwrap(),
        version: 2u32.to_be_bytes(),
        entries: file_names.len().to_be_bytes()[4..8].try_into().unwrap(),
    };
    content.extend(index_header.signature.to_vec());
    content.extend(index_header.version.to_vec());
    content.extend(index_header.entries.to_vec());

    // entries
    for (index, file_name) in file_names.iter().enumerate() {
        let metadata = fs::metadata(file_name).unwrap();

        let new_file_name = match file_name.strip_prefix("./") {
            Some(file_name) => file_name,
            None => file_name,
        };
        let index_entry = IndexEntry {
            ctime: metadata.st_ctime().to_be_bytes()[4..8].try_into().unwrap(),
            ctime_nsec: metadata.st_ctime_nsec().to_be_bytes()[4..8]
                .try_into()
                .unwrap(),
            mtime: metadata.st_mtime().to_be_bytes()[4..8].try_into().unwrap(),
            mtime_nsec: metadata.st_mtime_nsec().to_be_bytes()[4..8]
                .try_into()
                .unwrap(),
            dev: metadata.st_dev().to_be_bytes()[4..8].try_into().unwrap(),
            ino: metadata.st_ino().to_be_bytes()[4..8].try_into().unwrap(),
            mode: metadata.st_mode().to_be_bytes(),
            uid: metadata.st_uid().to_be_bytes(),
            gid: metadata.st_gid().to_be_bytes(),
            file_size: metadata.st_size().to_be_bytes()[4..8].try_into().unwrap(),
            oid: hash_list[index].clone(),
            // TODO: 正しく計算
            flags: new_file_name.len().to_be_bytes()[6..8].try_into().unwrap(),
            path: new_file_name.to_string(),
        };

        content.extend(index_entry.ctime.to_vec());
        content.extend(index_entry.ctime_nsec.to_vec());
        content.extend(index_entry.mtime.to_vec());
        content.extend(index_entry.mtime_nsec.to_vec());
        content.extend(index_entry.dev.to_vec());
        content.extend(index_entry.ino.to_vec());
        content.extend(index_entry.mode.to_vec());
        content.extend(index_entry.uid.to_vec());
        content.extend(index_entry.gid.to_vec());
        content.extend(index_entry.file_size.to_vec());
        let decoded_oid = hex::decode(index_entry.oid.clone()).unwrap();
        content.extend(decoded_oid);
        content.extend(index_entry.flags.to_vec());
        content.extend(index_entry.path.as_bytes().to_vec());
        let padding = 4 - (content.len() % 4);
        for _ in 0..padding {
            content.push(0);
        }
    }

    let mut file = File::create(".git/index").unwrap();
    file.write_all(&content).unwrap();
}
