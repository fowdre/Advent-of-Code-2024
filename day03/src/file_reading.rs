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

pub fn load_file(args: &[String]) -> Result<Vec<String>, String> {
    if args.len() < 3 {
        return Err("Not enough arguments".to_string());
    }

    match read_lines(&args[2]) {
        Ok(lines) => Ok(lines.map_while(Result::ok).collect()),
        Err(err) => Err(err),
    }
}
