use std::process::exit;

use super::env::WINDOWS_PATH;
use super::cp;
use super::get_filename::get_filename;
use super::get_current_dir::get_current_dir;

#[derive(Debug)]
pub enum Command {
    Copy(String), //  A guarda o path de saída
    Move(String),
    Remove,
}

impl Command {
    pub fn new(arg_cmd: &str, path: Option<&str>) -> Option<Command> {
        if arg_cmd == "cp" || arg_cmd == "copy" {
            match path {
                Some(value) => {
                    Some(Command::Copy(String::from(value)))
                },
                None => {
                    println!("Faltando argumentos\nUse: winbridge cp [local] [destino]");
                    exit(1);
                }
            }
        } 
        else if arg_cmd == "mv" || arg_cmd == "move" {
            match path {
                Some(value) => {
                    Some(Command::Move(String::from(value)))
                },
                None => {
                    println!("Faltando argumentos\nUse: winbridge mv [local] [destino]");
                    exit(1);
                }
            }
        }
        else if arg_cmd == "rm" || arg_cmd == "remove" {
            Some(Command::Remove)
        }
        else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub win_user_path: String, // Localização do Usuário Dentro do Disco C:
    pub command: Command, // Comando a ser executado
    pub win_path: String // Path dentro Windows
}

impl Config {
    pub fn new(user_path: &str, args: &Vec<String>) -> Config {
        let current_dir = match get_current_dir() {
            Some(value) => value,
            None => {
                eprintln!("Erro: get_current_dir()");
                exit(1);
            }
        };
        let win_path = format!("{}{}/{}", WINDOWS_PATH, user_path, args[2].clone());
        
        // parse segundo argumento
        let command_path: String = format!("{}/{}",current_dir, get_filename(&win_path));
        
        let cmd_option = Command::new(&args[1].clone(),Some(&command_path));
        let command = match cmd_option {
            Some(cmd) => cmd,
            None => {
                println!("\nError: invalid args");
                exit(1);
            }
        };

        Config {
            win_user_path: String::from(user_path), // Path do usuário Windows
            command,
            win_path, // Path dentro Windows
        }
    }

    pub fn execute(&self) {
        match &self.command {
            Command::Copy(value) => {
                println!("Copiando {} para {}", self.win_path, value);
                cp::copy(self);

            },
            Command::Move(value) => {
                println!("Movendo {} para {}", self.win_path, value);
            },
            Command::Remove => {
                println!("Removendo {}", self.win_path);
            }
        }
    }
}