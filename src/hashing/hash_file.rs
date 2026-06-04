use std::fs::File;
use crate::models::FileInfo;
use std::io::{BufReader,Read};
use blake3::{Hasher, Hash};
use anyhow::Result;

pub fn hash_file(file:&FileInfo) -> Result<Hash> {
    let open_file = File::open(&file.path)?;
    let mut reader = BufReader::new(open_file);
    let mut buffer = [0u8; 8192];
    let mut hasher = Hasher::new();

    loop {
        let n = reader.read(&mut buffer)?;

        if n == 0 {
            break;
        }

        hasher.update(&buffer[..n]);
    }

    let final_hash = hasher.finalize();

    Ok(final_hash)
}