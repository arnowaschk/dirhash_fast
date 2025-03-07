# Dirhash_fast

**Dirhash_fast**<br>
is a high-performance Rust tool designed to quickly compute cryptographic hashes (Blake3) of entire directory trees.
It supports parallel processing to scale effectively to large datasets, and works cross-platform (Linux, macOS, Windows).
It is much faster than the crate dirhash, and meant to be used more easily than merkle_hash.
Dirhash_fast can be used both as a standalone command-line application and as a library in Rust projects. <br><br>
**Usage (binary)**:<br>
Run `dirhash_fast /path/to/directory` to print the hash of the specified directory.<br><br>
**Usage (library)**:<br>Include <br>`dirhash_fast = "0"`<br> in your project's `Cargo.toml` under `[dependencies]`.<br>
Example usage in Rust: <br><br>
`use dirhash_fast::hash_directory;` <br>
`fn main() {` <br>
`    let hash = hash_directory("/path/to/directory".as_ref());`<br>
`    println!("Directory hash: {}", hash); `<br>
`    }`.<br><br>
**Installation**:<br>
From source: `cargo install --path .`, or from crates.io: `cargo install dirhash_fast`.<br><br>
**License**: Dirhash_fast is dual-licensed under either Apache-2.0 or MIT, at your option.<br><br>
© 2025 Arno Waschk.<br>
🚀 **Fast · Secure · Cross-platform**

