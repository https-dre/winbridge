mod module;
use std::env::args;
use std::process::exit;
use std::io::Result;

use module::read::*;
use module::functions::Mode;

use module::functions::set_mode;

const WINDOWS_PATH: &str = "/mnt/c";
const FS_CONFIG: &str = ".fs-config";

fn main() {
    println!("\n\nWindows File System to WSL!!\n\n");
    let args: Vec<String> = args().collect();
    //println!("{:?}", args);
    
    if args.len() < 3 {
        println!("Use:\n\tfs  [mode] [path]");
        exit(1);
    }

    let mode: Mode = set_mode(&args);

    match mode {
        Mode::Default => println!("Default mode"),
        Mode::Windows => {
            let path = args[2].clone();

            if verify_path(&path) && is_file(&path) {
                println!("{path} Is File!!");
            } else {
                println!("{path} Is Folder :(");
            }
        },
        Mode::None => println!("Please, select a mode\nUse: fs [mode] [path]")
    }

    let fs_config: Result<Vec<(String, String)>> = return_fs_config();
    match fs_config {
        Ok(lines) => {
            for value in lines.iter() {
                if value.0 == "windows_user_path" {
                    let mut windows_user_path = String::from(WINDOWS_PATH);
                    windows_user_path.push_str(&value.1);
                    println!("{windows_user_path}");
                }
            }
        },
        Err(e) => {
            println!("Erro no processo de leitura de arquivo de configuração\n{e}");
            exit(1);
        },
    }
}
// resolve para um array de tuplas string com (chave, valor)
fn return_fs_config() -> Result<Vec<(String, String)>> {
    match read_file(FS_CONFIG) {
        Ok(out) => {
            let lines: Vec<&str> = out.split("\n").collect();
            let mut result_lines: Vec<(String, String)> = Vec::new();

            for (index, line) in lines.iter().enumerate() {
                let mut key_value: (String, String) = (String::from(""), String::from(""));
                let mut equal_split: Vec<&str> = line.split("=").collect();
                
                // Verifica se há pelo menos dois elementos em equal_split
                if equal_split.len() >= 2 {
                    key_value.0 = String::from(equal_split[0]);
                    if equal_split[1].contains('\"') {
                        equal_split[1] = equal_split[1].trim_matches('"');
                    }
                     // tira as aspas de cada linha
                    key_value.1 = String::from(equal_split[1]);
                    result_lines.push(key_value);
                } else {
                    println!("Linha {} não contém '='", index);
                }
            }

            return Ok(result_lines);
        }
        Err(e) => Err(e),
    }
    
}