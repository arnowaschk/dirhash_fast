# Dirhash_fast

**Dirhash_fast** is a high-performance Rust tool designed to quickly compute cryptographic hashes (Blake3) of entire directory trees.
It supports parallel processing to scale effectively to large datasets, and works cross-platform (Linux, macOS, Windows).
It is much faster than the crate dirhash, and meant to be used more easily than merkle_hash.
Dirhash_fast can be used both as a standalone command-line application and as a library in Rust projects. 
**Usage (binary)**:
Run `dirhash /path/to/directory` to print the hash of the specified directory.
**Usage (library)**: Include `dirhash = "0.1"` in your project's `Cargo.toml` under `[dependencies]`.
Example usage in Rust: `use dirhash::hash_directory; fn main() { let hash = hash_directory("/path/to/directory".as_ref()); println!("Directory hash: {}", hash); }`.
**Installation**: From source: `cargo install --path .`, or from crates.io: `cargo install dirhash`.
**License**: Dirhash is dual-licensed under either Apache-2.0 or MIT, at your option.
© 2025 Arno Waschk.
🚀 **Fast · Secure · Cross-platform**

