use core::panic;
use std::collections;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::os::linux::fs::MetadataExt;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use byteorder::{BigEndian, ByteOrder};

use crate::util;

fn travel_dir(
    file_name: impl AsRef<Path>,
    file_path_list: &mut Vec<PathBuf>,
    hash_list: &mut Vec<String>,
) -> io::Result<()> {
    if !fs::metadata(&file_name)?.is_dir() {
        let hash = generate_blob_object(&file_name)?;
        file_path_list.push(file_name.as_ref().to_path_buf());
        hash_list.push(hash);
        return Ok(());
    }

    // 再帰的にaddする
    for entry in fs::read_dir(file_name)? {
        let path = entry?.path();
        if path.starts_with("./.git") {
            continue;
        }

        if path.is_dir() {
            travel_dir(&path, file_path_list, hash_list)?;
            continue;
        }
        let hash = generate_blob_object(&path)?;
        file_path_list.push(path);
        hash_list.push(hash);
    }
    Ok(())
}

pub fn add(file_names: &[PathBuf]) -> anyhow::Result<()> {
    let mut hash_list = Vec::new();
    let mut file_path_list = Vec::new();
    for file_name in file_names {
        travel_dir(file_name, &mut file_path_list, &mut hash_list)?;
    }
    update_index(&file_path_list, &hash_list)
}

fn generate_blob_object(file_name: impl AsRef<Path>) -> Result<String, io::Error> {
    let contents = fs::read_to_string(file_name)?;
    let file_length = contents.len();

    // データの準備
    let header = format!("blob {file_length}\0");
    let hash = util::compress::hash(format!("{header}{contents}").as_bytes());

    // ファイルの準備
    let file_directory = format!(".git/objects/{}", &hash[0..2]);
    let file_path = format!("{}/{}", file_directory, &hash[2..]);
    let mut file = util::path::create_nested_file(file_path)?;

    // zlib圧縮
    let contents_will_be_compressed = format!("{header}{contents}");
    let compressed_contents = util::compress::with_zlib(contents_will_be_compressed.as_bytes())?;

    // ファイルに書き込み
    file.write_all(&compressed_contents)?;

    Ok(hash)
}

#[derive(Clone)]
struct IndexEntrySummary {
    index_entry: Vec<u8>,
    path: PathBuf,
}

// 既存のentriesと新しく追加されるentriesをmergeする
// 順番を変えるとファイルが削除されて新しく作成されたとみなされてしまうため、順番は変わらないようにする
fn merge_entries(
    exists: &[IndexEntrySummary],
    new_entries: Vec<IndexEntrySummary>,
) -> Vec<IndexEntrySummary> {
    let exist_paths: collections::HashSet<_> = exists.iter().map(|x| x.path.clone()).collect();
    let new_paths: collections::HashSet<_> = new_entries.iter().map(|x| x.path.clone()).collect();

    let common_paths: collections::HashSet<_> = exist_paths.intersection(&new_paths).collect();

    let mut result = Vec::<IndexEntrySummary>::new();

    for entry in exists.iter().cloned() {
        if !common_paths.contains(&entry.path) {
            result.push(entry);
            continue;
        }
        match new_entries.iter().find(|&x| x.path == entry.path).cloned() {
            Some(item) => result.push(item),
            None => panic!("not found"),
        };
    }
    for entry in new_entries {
        if !common_paths.contains(&entry.path) {
            result.push(entry);
        }
    }

    result
}

fn decode_index_file() -> Option<Vec<IndexEntrySummary>> {
    let Ok(mut file) = File::open(".git/index") else {
        return None;
    };
    let mut content = Vec::new();
    let mut index_entry_summaries = Vec::<IndexEntrySummary>::new();
    file.read_to_end(&mut content).unwrap();

    // entriesを上から1 entryずつ消費していく
    let entry_count = BigEndian::read_u32(&content[8..12]);
    let mut entries = &content[12..];
    for _ in 0..entry_count {
        let (next_byte, index_entry_summary) = decode_index_entry(entries);
        index_entry_summaries.push(index_entry_summary);
        entries = &entries[next_byte..];
    }

    Some(index_entry_summaries)
}

fn decode_index_entry(entry: &[u8]) -> (usize, IndexEntrySummary) {
    let flags = BigEndian::read_u16(&entry[60..62]);
    let file_path_end_byte = (62 + flags) as usize;
    let path: &Path = OsStr::from_bytes(&entry[62..file_path_end_byte]).as_ref();

    let padding = 4 - (file_path_end_byte % 4);
    let next_byte = file_path_end_byte + padding;
    let index_entry_summary = IndexEntrySummary {
        index_entry: entry[..next_byte].to_vec(),
        path: path.to_path_buf(),
    };

    (next_byte, index_entry_summary)
}

fn update_index(file_names: &[PathBuf], hash_list: &[String]) -> anyhow::Result<()> {
    // 既にindex fileが存在したらそれを読み込み、entriesをdecode
    // headerは新しく作る(entryの数が違うため)

    // 更新されるファイルのentries
    let exists = decode_index_file();

    // 新しく追加されるファイルのentries
    let mut new_entries = Vec::<IndexEntrySummary>::new();

    for (index, file_name) in file_names.iter().enumerate() {
        let metadata = fs::metadata(file_name)?;

        let new_file_name = &file_name.strip_prefix("./").unwrap_or(file_name);
        let change_time = &metadata.st_ctime().to_be_bytes()[4..8];
        let change_time_nsec = &metadata.st_ctime_nsec().to_be_bytes()[4..8];
        let modification_time = &metadata.st_mtime().to_be_bytes()[4..8];
        let modification_time_nsec = &metadata.st_mtime_nsec().to_be_bytes()[4..8];
        let dev = &metadata.st_dev().to_be_bytes()[4..8];
        let ino = &metadata.st_ino().to_be_bytes()[4..8];
        let mode = &metadata.st_mode().to_be_bytes();
        let user_id = &metadata.st_uid().to_be_bytes();
        let group_id = &metadata.st_gid().to_be_bytes();
        let file_size = &metadata.st_size().to_be_bytes()[4..8];
        let oid = &hash_list[index];
        let decoded_oid = hex::decode(oid)?;
        let decoded_oid_slice = decoded_oid.as_slice();
        // TODO: 正しく計算
        let flags = &new_file_name.as_os_str().len().to_be_bytes()[6..8];
        let path = new_file_name.to_path_buf();

        let mut content: Vec<u8> = [
            change_time,
            change_time_nsec,
            modification_time,
            modification_time_nsec,
            dev,
            ino,
            mode,
            user_id,
            group_id,
            file_size,
            decoded_oid_slice,
            flags,
            path.as_os_str().as_bytes(),
        ]
        .concat();

        let padding = 4 - (content.len() % 4);
        content.resize(content.len() + padding, 0);

        let index_entry_summary = IndexEntrySummary {
            index_entry: content,
            path,
        };
        new_entries.push(index_entry_summary);
    }

    let merged_entries = match exists {
        Some(e) => merge_entries(&e, new_entries),
        None => new_entries,
    };

    // header
    let signature = b"DIRC";
    let version = &2u32.to_be_bytes();
    let entrie_count = &merged_entries.len().to_be_bytes()[4..8];

    let mut contents: Vec<u8> = [signature, version, entrie_count].concat();

    // entries
    for entry in merged_entries {
        contents.extend(entry.index_entry);
    }

    let mut file = File::create(".git/index")?;
    file.write_all(&contents)?;
    Ok(())
}
