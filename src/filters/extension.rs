use crate::models::FileInfo;

pub fn matches_extension(file:&FileInfo,ext:&Option<Vec<String>>) -> bool {
    match ext {
        Some(value) => {
             let ans = match file.path.extension() {
               Some(v) =>{
                    value.contains(&v.to_string_lossy().to_lowercase())
               },
                 None => false
            };

            ans
        }
        None => true
    }
}