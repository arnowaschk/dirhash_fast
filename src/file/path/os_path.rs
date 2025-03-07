use std::ffi::{OsString,OsStr};
use std::path::Path;

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

pub fn get_encoded_path(path: &Path) -> Vec<u8> {
    let os_string = path.as_os_str().to_os_string();
    
    #[cfg(unix)]
    {
        return os_string.as_bytes().to_vec();
    }
    
    #[cfg(windows)]
    {
        return os_string.encode_wide()
            .flat_map(|w| w.to_le_bytes())
            .collect();
    }
}

pub fn decode_encoded_path(encoded: &[u8]) -> OsString {
    #[cfg(unix)]
    {
        return OsStr::from_bytes(encoded).to_os_string();
    }
    
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStringExt;
        let utf16: Vec<u16> = encoded
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
        return OsString::from_wide(&utf16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_encoding_decoding_path() {
        let sample_path = PathBuf::from("/tmp/some/test/path");
        let encoded_path = get_encoded_path(&sample_path);
        let decoded_path = decode_encoded_path(&encoded_path);
        
        assert_eq!(sample_path, decoded_path, "Decoded path should match original");
    }
}