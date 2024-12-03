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
    let mut solution = 0;

    for line in input {
        let parsed_line: Vec<_> = line
            .split(" ")
            .map(|nbr| nbr.parse::<i128>().unwrap())
            .collect();

        let mut ok = true;
        let mut flag = parsed_line[0] <= parsed_line[1];
        for i in 0..parsed_line.len() - 1 {
            let new_flag = parsed_line[i] <= parsed_line[i + 1];
            if flag != new_flag {
                ok = false;
                flag = new_flag;
                break;
            }
            flag = new_flag;
            let diff = parsed_line[i].abs_diff(parsed_line[i + 1]);
            if !(diff == 1 || diff == 2 || diff == 3) {
                ok = false;
                break;
            }
        }

        if ok {
            solution += 1;
        }
    }

    solution
}

fn part2_solution(input: &[String]) -> u128 {
    let mut solution = 0;

    for line in input {
        let parsed_line: Vec<_> = line
            .split(" ")
            .map(|nbr| nbr.parse::<i128>().unwrap())
            .collect();

        let mut ok = true;
        let mut flag = parsed_line[0] <= parsed_line[1];
        let mut got_bad_level = false;
        println!("line {:?}", parsed_line);
        for i in 0..parsed_line.len() - 1 {
            println!("{} {}", parsed_line[i], parsed_line[i + 1]);
            let new_flag = parsed_line[i] <= parsed_line[i + 1];
            if flag != new_flag {
                if !got_bad_level {
                    got_bad_level = true;
                    flag = new_flag;
                    continue;
                }
                ok = false;
                flag = new_flag;
                break;
            }
            let diff = parsed_line[i].abs_diff(parsed_line[i + 1]);
            if !(diff == 1 || diff == 2 || diff == 3) {
                if !got_bad_level {
                    got_bad_level = true;
                    continue;
                }
                ok = false;
                break;
            }
            flag = new_flag;
        }

        if ok {
            solution += 1;
        }
    }

    solution
}
