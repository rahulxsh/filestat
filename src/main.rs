use std::ffi::OsStr;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let path = Path::new("./src");
    scan(path);
}

fn scan(path: &Path) {
    let mut total_file = 0;
    let mut total_dir = 0;

    for entry in WalkDir::new(path) {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };

        let file_type = entry.file_type();

        if file_type.is_dir() {
            total_dir += 1;
            println!("[DIR] {}", entry.path().display());
        }

        if file_type.is_file() {
            total_file += 1;
            println!("[FILE] {}", entry.path().display());

            if let Some(ext) = entry.path().extension() {
                if ext == OsStr::new("rs") {
                    println!("Rust File -> {}", entry.path().display());
                }
            }
        }
    }

    println!("Total files: {}", total_file);
    println!("Total dirs: {}", total_dir);
}