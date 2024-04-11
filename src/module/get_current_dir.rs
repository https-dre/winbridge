use std::env;

pub fn get_current_dir() -> Option<String> {
    if let Ok(current_dir) = env::current_dir() {
        if let Some(path) = current_dir.to_str() {
            return Some(String::from(path));
        }
    }
    None
}