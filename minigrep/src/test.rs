use std::env;
use std::fs;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let path_or_file = Path::new(&args[2]);


    

}


fn is_dir_path(path_or_file: &String) -> bool {
    let len = path_or_file.len();
    if len < 3 {return true}
    if path_or_file[path_or_file.len()-3..] == "txt".to_string() {return false}
    true
}

// En fonction du chemin du fichier en entrée, un Array est retourné avec les index des occurence de la query 

fn grepFile (file_path: &String, query: &String ) -> [u16] {

}

// En fonction du chemin du folder en entrée, une HashMap est retourné avec comme clé le chemin du fichier texte et en objet associé les occurrence de la query 

fn grepFolder(dir_path: &String, query: &String) -> [u16] {
    let read = fs::read_dir(dir_path).unwrap();

    let hashMapResult = HashMap::new();
    for item in read {
        if is_dir_path(item.path()){grepFolder(item.path(), query)}
        hashMapResult.insert(item, grepFile((item.path()), query));
    }
    return hashMapResult;
}