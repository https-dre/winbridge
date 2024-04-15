mod module;
use std::env::args;
use std::process::exit;
use module::config::Config;
use module::get_file_config::get_file_config;

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
