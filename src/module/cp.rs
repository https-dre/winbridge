use std::fs;
use std::process::exit;
use fs_extra::dir::{copy as fs_extra_copy, CopyOptions};

use super::config::*;
use super::read::*;

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

pub fn copy(config: &Config) {
    if verify_path(&config.win_path) {
        match &config.command {
            Command::Copy(value) => {
                if is_file(&config.win_path) {
                    fs::copy(&config.win_path, value).expect("Falha ao copiar arquivo");
                } else {
                    println!("\nNão é um arquivo, tentando copiar...\n");
                    let options = CopyOptions::new();
                    fs_extra_copy(&config.win_path, value, &options).expect("Falha ao copiar diretório");
                }
            },
            _ => {
                println!("O comando não é uma cópia.");
                exit(1);
            }
        }
    } else {
        println!("{} não existe. Verifique se o arquivo foi movido ou renomeado", config.win_path);
        exit(1);
    }
}
