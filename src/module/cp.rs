use std::fs;
use std::path::Path;
use std::process::exit;

mod config;
use config::*;

/*
    pub enum Command {
        Copy(String),
        Move(String),
        Remove,
        Default
    }
    pub struct Config {
        win_user_path: String,
        command: Command,
        path: String,
    }
*/

pub fn copy(config: Config) {
    fs::copy(config.path, config.command);
}
