// Uses template from commit <>

mod file_reading;
use file_reading::load_file;

fn main() {
    match load_file(std::env::args().nth(1)) {
        Ok((file_name, input)) => {
            println!("[Contents of file file: {file_name}]");
            for line in &input {
                println!("{line}")
            }
        }
        Err(message) => {
            eprintln!("Error: {}", message);
        }
    }
}

fn part1_solution(input: Vec<String>) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// To check if modifications between different steps don't invalidate previous ones
    fn breaking_changes_test() {
        let input_path = "../inputs/day/small.txt";
        let (_, input) = load_file(Some(input_path.to_string())).expect("Failed to load file");

        assert_eq!(input, input);
    }
}
