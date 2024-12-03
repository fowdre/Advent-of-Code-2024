// Uses template from commit <08a304cea143e00a0a51cf72f73e966eec5cd29e>

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

fn are_all_nums_same_sign(nums: &[i128]) -> bool {
    // let mut iter = nums.iter();

    // iter.all(|num| num.is_positive()) || iter.all(|num| num.is_negative()) // don't work becausee 0 is positive & negative at the same time

    let mut iter = nums.iter();
    let sign = iter.find(|&&num| num != 0).unwrap_or(&1).signum();

    iter.all(|num| num.signum() == sign)
}

fn part1_solution(input: &[String]) -> u128 {
    let mut safe_reports_count = 0;

    for line in input {
        let parsed_line: Vec<_> = line
            .split_whitespace()
            .map(|num| num.parse::<i128>().unwrap())
            .collect();

        println!("\nline {line}");
        let pairs: Vec<&[i128]> = parsed_line.windows(2).collect();
        println!("pairs {pairs:?}");
        let differences: Vec<i128> = pairs.iter().map(|pairs| pairs[1] - pairs[0]).collect();
        if differences.iter().any(|&diff| diff == 0) {
            continue;
        }

        println!("diffs {differences:?}");
        if !are_all_nums_same_sign(&differences) {
            continue;
        }

        let unsafe_reports: Vec<i128> = differences
            .into_iter()
            .filter(|diff| {
                let abs = diff.abs();

                abs != 1 && abs != 2 && abs != 3
            })
            .collect();

        println!("unsafe {unsafe_reports:?}");

        if !unsafe_reports.is_empty() {
            continue;
        }

        safe_reports_count += 1;
    }

    safe_reports_count
}

fn part2_solution(input: &[String]) -> u128 {
    let mut safe_reports_count = 0;

    for line in input {
        let parsed_line: Vec<_> = line
            .split_whitespace()
            .map(|num| num.parse::<i128>().unwrap())
            .collect();
        println!("\nline {line}");

        for i in 0..parsed_line.len() + 1 {
            // Test by removing one element at a time, starts with +1 to test the entire list (0 will mean check without removing an element)
            let mut cloned_line = parsed_line.clone();
            if i != 0 {
                cloned_line.remove(i - 1);
            }

            let pairs: Vec<&[i128]> = cloned_line.windows(2).collect();
            println!("pairs {pairs:?}");
            let differences: Vec<i128> = pairs.iter().map(|pairs| pairs[1] - pairs[0]).collect();
            if differences.iter().any(|&diff| diff == 0) {
                continue;
            }

            println!("diffs {differences:?}");
            if !are_all_nums_same_sign(&differences) {
                continue;
            }

            let unsafe_reports: Vec<i128> = differences
                .into_iter()
                .filter(|diff| {
                    let abs = diff.abs();

                    abs != 1 && abs != 2 && abs != 3
                })
                .collect();

            println!("unsafe {unsafe_reports:?}");

            if !unsafe_reports.is_empty() {
                continue;
            }

            safe_reports_count += 1;
            break;
        }
    }

    safe_reports_count
}
