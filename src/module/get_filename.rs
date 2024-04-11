pub fn get_filename(path: &String) -> String{
    let mut file_name = String::from(path);
    let lista: Vec<&str> = file_name.split('/').collect();
    file_name = lista[lista.len() - 1].to_string();
    file_name
}
