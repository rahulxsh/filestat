use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use blake3::{Hasher, Hash};
use anyhow::Result;

pub fn hash_file(file:&PathBuf) -> Result<Hash> {
    let mut open_file = File::open(&file)?;
    let mut hasher = Hasher::new();

    hasher.update_reader(&mut open_file)?;

    let final_hash = hasher.finalize();

    Ok(final_hash)
}

pub fn hash_file_partial(path:&PathBuf) -> Result<Hash> {
    let mut hasher = Hasher::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut handle = reader.take(4096);


    let mut buffer = [0; 4096];

    let bytes_count = handle.read(&mut buffer)?;
    hasher.update(&buffer[..bytes_count]);

    Ok(hasher.finalize())
}