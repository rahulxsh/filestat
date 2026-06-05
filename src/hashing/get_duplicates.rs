use std::collections::HashMap;
use std::path::{PathBuf};
use anyhow::Result;
use blake3::Hash;
use crate::hashing::hash_file::{hash_file, hash_file_partial};
use rayon::prelude::*;


pub fn get_full_duplicates(size_map:HashMap<u64,Vec<PathBuf>>) -> Result<HashMap<Hash, Vec<PathBuf>>> {
    let mut final_duplicates: HashMap<Hash, Vec<PathBuf>> = HashMap::new();
    let mut total_hashes_computed = 0;

    for (_size,file_paths) in size_map {

        if file_paths.len() > 1 {
            let partial_hash_map:HashMap<Hash,Vec<PathBuf>> = file_paths
                .par_iter()
                .filter_map(|v| {
                    hash_file_partial(&v).ok().map(|phash| (phash,v))
                })
                .fold(
                    || HashMap::new(),
                    | mut local_map:HashMap<Hash,Vec<PathBuf>>, (phash,v) |  {
                        local_map.entry(phash).or_default().push(v.clone());
                        local_map
                    }
                )
                .reduce(
                    || HashMap::new(),
                    | mut map1, map2 | {
                        for (k,v) in map2 {
                            map1.entry(k).or_default().extend(v);
                        }
                        map1
                    }
                );

            let results: Vec<Result<(Hash, PathBuf)>> = partial_hash_map
                .par_iter()
                .filter(|(_, candidate_paths)| candidate_paths.len() > 1)
                .flat_map(|(_, candidate_paths)| candidate_paths)
                .map(|path| {
                    // If hash_file returns an anyhow::Error, the ? works perfectly here
                    let f_hash = hash_file(path)?;
                    Ok((f_hash, path.clone()))
                })
                .collect();

            for res in results {
                let (f_hash, path) = res?; // Triggers anyhow's early return if any file failed to hash
                final_duplicates.entry(f_hash).or_default().push(path);
                total_hashes_computed += 1;
            }
        }
    }

    println!("Total full file hashes computed: {}", total_hashes_computed);

    Ok(final_duplicates)
 }