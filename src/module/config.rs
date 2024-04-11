use std::process::exit;

pub enum Command {
    Copy,
    Move,
    Remove,
    Default
}

impl Command {
    fn new(arg_cmd: &str) -> Option<Command> {
        if arg_cmd == "cp" || arg_cmd == "copy" {
            return Some(Command::Copy);
        } 
        else if arg_cmd == "mv" || arg_cmd == "move" {
            return Some(Command::Move);
        }
        else if arg_cmd == "rm" || arg_cmd == "remove" {
            return Some(Command::Remove);
        }
        else {
            return None;
        }
    }
}

pub struct Config {
    win_user_path: String,
    command: Command,
    path: String
}

impl Config {
    fn new(user_path: &str, args: &Vec<String>) -> Config {
        let cmd_option = Command::new(&args[1].clone());
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