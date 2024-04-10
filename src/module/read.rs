use std::path::Path;
use std::fs::read_to_string;
use std::io::Result;


pub fn verify_path(path_str: &String) -> bool {
    let path: &Path = Path::new(path_str);
    if path.exists() {
        return true;
    }
    return false;
}

pub fn is_file(path_str: &String) -> bool {
    let path = Path::new(path_str);
    if path.is_file() {
        return true;
    }
    println!("Not a File");
    return false;
}

pub fn read_file(path_str: &str) -> Result<String> {
    let path = Path::new(path_str);
    read_to_string(path)
}