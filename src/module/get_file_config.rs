use super::env::*;
use super::read::*;
use std::io::Result;

pub fn get_file_config() -> Result<Vec<(String, String)>> {
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
