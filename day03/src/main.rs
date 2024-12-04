// Uses template from commit <>

mod file_reading;
use file_reading::load_file;
use regex::Regex;

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
    let input = input.join("\n");
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut muls = Vec::new();
    for mul in re.captures_iter(&input) {
        let digit_1 = &mul[1].parse::<u128>().unwrap();
        let digit_2 = &mul[2].parse::<u128>().unwrap();

        muls.push(digit_1 * digit_2);
    }

    muls.iter().sum()
}

fn part2_solution(input: &[String]) -> u128 {
    let mut input = input.join("\n");

    let mut search_substrs = Vec::new();
    let mut mul_enabled = true;
    loop {
        if mul_enabled {
            let pos = input.find("don't()");
            if let Some(pos) = pos {
                mul_enabled = false;
                search_substrs.push(input[..pos].to_string());
                input = input[pos + 7..].to_string();
            } else {
                search_substrs.push(input);
                break;
            }
        } else {
            let pos = input.find("do()");
            if let Some(pos) = pos {
                mul_enabled = true;
                input = input[pos + 4..].to_string();
            } else {
                break;
            }
        }
    }

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut muls = Vec::new();
    for substr in search_substrs {
        for mul in re.captures_iter(&substr) {
            let digit_1 = &mul[1].parse::<u128>().unwrap();
            let digit_2 = &mul[2].parse::<u128>().unwrap();

            muls.push(digit_1 * digit_2);
        }
    }

    muls.iter().sum()
}
