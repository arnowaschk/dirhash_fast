use std::path::Path;
use std::fs::File;
use std::io::{BufReader,Read};
use walkdir::WalkDir;
use rayon::prelude::*;

use crate::file::path::os_path::get_encoded_path;


fn hash_file(path: &Path) -> Option<String> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);

    let mut hasher = blake3::Hasher::new();
    
    let mut buffer = [0; 8192];
    
    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 { break; }
        hasher.update(&buffer[..bytes_read]);
    }
    
    Some(hasher.finalize().to_hex().to_string())
}

pub fn hash_directory(dir: &Path) -> String {
    let mut hasher = blake3::Hasher::new();
    
    let mut entries: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    entries.sort_by(|a, b| a.path().cmp(b.path()));

    let hashes: Vec<_> = entries
        .into_par_iter()
        .filter_map(|entry| hash_file(entry.path()).map(|h| (get_encoded_path(entry.path()), h)))
        .collect();
    
    for (path, hash) in hashes {
        hasher.update(&path);
        hasher.update(hash.as_bytes());
    }
    
    hasher.finalize().to_hex().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    
    #[test]
    fn test_hash_directory() {
        let dir = tempdir().expect("Failed to create temp directory");
        let file_path = dir.path().join("testfile.txt");
        
        let mut file = File::create(&file_path).expect("Failed to create temp file");
        writeln!(file, "Hello, world!").expect("Failed to write to temp file");
        
        let hash1 = hash_directory(dir.path());
        assert!(!hash1.is_empty(), "Hash should not be empty");
        
        let hash2 = hash_directory(dir.path());
        assert_eq!(hash1, hash2, "Hashes should be deterministic");
    }
}
