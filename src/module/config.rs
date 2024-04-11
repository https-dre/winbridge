use std::process::exit;

#[derive(Debug)]
pub enum Command {
    Copy(String),
    Move(String),
    Remove,
    Default
}

impl Command {
    pub fn new(arg_cmd: &str, path: Option<&str>) -> Option<Command> {
        if arg_cmd == "cp" || arg_cmd == "copy" {
            match path {
                Some(value) => {
                    Some(Command::Copy(String::from(value)))
                },
                None => {
                    println!("Argumento faltando para Config.command");
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
                    println!("Argumento faltando para Config.command");
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
    win_user_path: String,
    command: Command,
    path: String
}

impl Config {
    pub fn new(user_path: &str, args: &Vec<String>) -> Config {
        let cmd_option = Command::new(&args[1].clone(),Some(&args[3].clone()));
        let command = match cmd_option {
            Some(cmd) => cmd,
            None => {
                println!("\nError: invalid args");
                exit(1);
            }
        };

        Config {
            win_user_path: String::from(user_path),
            command,
            path: args[2].clone(),
        }
    }
}