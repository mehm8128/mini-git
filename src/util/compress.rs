use std::io::Write;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

pub fn with_zlib(str: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(str)?;
    e.finish()
}

pub fn hash(str: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(str);
    let result = hasher.finalize();
    format!("{result:x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_zlib() {
        let str = "test".to_string();
        let result = with_zlib(str.as_bytes()).unwrap();
        assert_eq!(result, vec![120, 156, 43, 73, 45, 46, 1, 0, 4, 93, 1, 193]);
    }

    #[test]
    fn test_hash() {
        let str = "test".to_string();
        let result = hash(str.as_bytes());
        assert_eq!(result, "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3");
    }
}
