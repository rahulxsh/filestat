use crate::models::FileInfo;
use crate::utils::size_parser::parse;

pub fn min_file_size(file:&FileInfo, min_size:&Option<String>) -> bool {
    match min_size {
        Some(v) => {
            let size = parse(v).unwrap_or(1);

            file.size >= size
        }
        None => true
    }
}

pub fn max_file_size(file:&FileInfo,max_size:&Option<String>) -> bool {
    match max_size {
        Some(v) =>{
            let size = parse(v).unwrap_or(1_000_000);

            file.size <= size
        }
        None => true
    }
}