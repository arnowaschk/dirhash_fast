use dirhash_fast::file::dir::dirhash::hash_directory;
use std::path::Path;

fn main() {
    let path = std::env::args().nth(1).expect("Bitte ein Verzeichnis angeben");
    let dir_path = Path::new(&path);
    
    if dir_path.is_dir() {
        let hash = hash_directory(dir_path);
        println!("Hash für {}: {}\n", path, hash);
    } else {
        eprintln!("{} ist kein gültiges Verzeichnis", path);
    }
}