use std::collections::HashMap;
use std::path::{PathBuf};
use anyhow::Result;
use blake3::Hash;
use crate::hashing::hash_file::{hash_file, hash_file_partial};

// pub fn get_duplicates(files:HashMap<Hash,Vec<PathBuf>>) -> Result<HashMap<Hash,Vec<PathBuf>>> {
//     let mut map:HashMap<Hash,Vec<PathBuf>> = HashMap::new();
//     let mut count = 0;
//
//     for (_key,file_paths) in files {
//         if file_paths.len() > 1 {
//             for f  in file_paths {
//                 let hash = hash_file(&f)?;
//                 map.entry(hash).or_default().push(f);
//                 count+=1;
//             }
//         }
//     }
//
//     println!("Total files hash count:{}",count);
//
//     Ok(map)
// }

// pub fn get_partial_duplicates(files:HashMap<u64,Vec<PathBuf>>) -> HashMap<Hash,Vec<PathBuf>> {
//     let mut partial_hash_map: HashMap<Hash, Vec<PathBuf>> = HashMap::new();
//
//
//     for (_key,file_paths) in files {
//        if file_paths.len() > 1 {
//            for path in file_paths {
//                // hash_file_partial reads up to 4KB.
//                if let Ok(p_hash) = hash_file_partial(&path) {
//                    partial_hash_map.entry(p_hash).or_default().push(path);
//                }
//            }
//        }
//     }
//
//     partial_hash_map
// }

pub fn get_full_duplicates(size_map:HashMap<u64,Vec<PathBuf>>) -> Result<HashMap<Hash, Vec<PathBuf>>> {
    let mut final_duplicates: HashMap<Hash, Vec<PathBuf>> = HashMap::new();
    let mut total_hashes_computed = 0;

    for (_size,file_paths) in size_map {

        if file_paths.len() > 1 {
            let mut partial_hash_map: HashMap<Hash, Vec<PathBuf>> = HashMap::new();
            for path in file_paths {
                if let Ok(p_hash) = hash_file_partial(&path) {
                    partial_hash_map.entry(p_hash).or_default().push(path);
                }
            }

            for (_p_hash,candidate_paths) in partial_hash_map {
                if candidate_paths.len() > 1 {
                    for path in candidate_paths {
                        let f_hash = hash_file(&path)?;
                        final_duplicates.entry(f_hash).or_default().push(path);
                        total_hashes_computed += 1;
                    }
                }
            }
        }
    }

    println!("Total full file hashes computed: {}", total_hashes_computed);

    Ok(final_duplicates)
 }