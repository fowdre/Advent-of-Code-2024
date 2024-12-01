// Uses template from commit <>

mod file_reading;
use file_reading::load_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match load_file(&args) {
        Ok(input) => {
            if args[1] == "1" {
                let part1_result = part1_solution(&input);

                println!("Part 1 solution: {}", part1_result);
            } else if args[1] == "2" {
                let part2_result = part2_solution(&input);

                println!("Part 2 solution: {}", part2_result);
            } else {
                eprintln!("Invalid argument. Please use 1 or 2.");
                std::process::exit(1);
            }
        }
        Err(message) => {
            eprintln!("Error: {}", message);
            std::process::exit(1);
        }
    }
}

fn part1_solution(input: &[String]) -> u128 {
    todo!("Implement solution for part 1")
}

fn part2_solution(input: &[String]) -> u128 {
    todo!("Implement solution for part 2")
}
