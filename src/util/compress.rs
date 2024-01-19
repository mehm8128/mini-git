use std::io::Write;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

pub fn zlib_compress(str: &[u8]) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(str).unwrap();
    e.finish().unwrap()
}

pub fn hash(str: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(str);
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zlib_compress() {
        let str = "test".to_string();
        let result = zlib_compress(str.as_bytes());
        assert_eq!(result, vec![120, 156, 43, 73, 45, 46, 1, 0, 4, 93, 1, 193]);
    }

    #[test]
    fn test_hash() {
        let str = "test".to_string();
        let result = hash(str.as_bytes());
        assert_eq!(result, "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3");
    }
}
