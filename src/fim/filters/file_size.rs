use crate::fim::models::FileInfo;
use crate::fim::utils::size_parser::parse;

pub fn  matches_min_file_size(file:&FileInfo, min_size:&Option<String>) -> bool {
    match min_size {
        Some(v) => {
            let size = parse(v).unwrap_or(1);

            file.size >= size
        }
        None => true
    }
}

pub fn  matches_max_file_size(file:&FileInfo,max_size:&Option<String>) -> bool {
    match max_size {
        Some(v) =>{
            let size = parse(v).unwrap_or(1_000_000);

            file.size <= size
        }
        None => true
    }
}