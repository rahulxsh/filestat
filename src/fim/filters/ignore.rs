use crate::fim::models::FileInfo;

#[allow(unused)]
pub fn matches_ignore(file:&FileInfo,ignore:&Option<Vec<String>>) -> bool {
    match ignore {
        Some(value) =>{
            if value.is_empty() {
                return true
            }
            let path = file.path.to_string_lossy();

            !value.iter().any(|pattern| {
                path.contains(pattern)
            })
        }
        None => true
    }
}