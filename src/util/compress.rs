use std::io::Write;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

pub fn zlib_compress(str: String) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(str.as_bytes()).unwrap();
    e.finish().unwrap()
}

pub fn hash(str: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(str);
    let result = hasher.finalize();
    format!("{:x}", result)
}
