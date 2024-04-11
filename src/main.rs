mod module;
use std::env::args;
use std::process::exit;
use std::io::Result;

use module::read::*;
use module::config::Config;
use module::env::*;

fn main() {
    println!("\n\nWindows Bridge to WSL\n\n");
    let args: Vec<String> = args().collect();
    //println!("{:?}", args);
    
    if args.len() < 2 {
        println!("Use:\n\twinbridge [function] [path]");
        exit(1);
    }
    let file_config = get_file_config();
    match file_config {
        Ok(config_lines) => {
            let mut win_user_path = String::from("default");
            
            for value in config_lines.iter() {
                if value.0 == "windows_user_path" {
                    win_user_path = String::from(&value.1);
                }
            }

            // Verificando Variável Path do Usuário No Windows
            if win_user_path == "default" {
                println!("windows_user_path faltando");
                exit(1);
            }

            // Configuração do Comando a Ser Executado
            let runtime_config = Config::new(&win_user_path, &args);

            runtime_config.execute();

            println!("\nSua runtime-config {:?}", runtime_config);
        },
        Err(e) => {
            println!("Erro ao ler arquivo de configuração!\n {e}");
            exit(1);
        }
    }
}
// resolve para um array de tuplas string com (chave, valor)
fn get_file_config() -> Result<Vec<(String, String)>> {
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
