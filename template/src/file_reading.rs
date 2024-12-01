use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(file_path: &P) -> Result<io::Lines<io::BufReader<File>>, String>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.is_dir() {
        return Err("Path is a directory".to_string());
    }

    Ok(io::BufReader::new(file).lines())
}

pub fn load_file(file_path: Option<String>) -> Result<(String, Vec<String>), String> {
    match file_path {
        Some(path) => match read_lines(&path) {
            Ok(lines) => Ok((path, lines.map_while(Result::ok).collect())),
            Err(err) => Err(err),
        },
        None => Err("Not enough arguments".to_string()),
    }
}
