// Uses template from commit <8e45059a05d62cb530d9c73498f2d84eacbde524>

mod file_reading;
use file_reading::load_file;

fn main() {
    match load_file(std::env::args().nth(1)) {
        Ok((file_name, input)) => {
            // println!("[Contents of file file: {file_name}]");
            // for line in &input {
            //     println!("{line}")
            // }

            // let part1_result = part1_solution(&input);
            // println!("Part 1 solution: {part1_result}");

            let part2_result = part2_solution(&input);
            println!("Part 2 solution: {part2_result}");
        }
        Err(message) => {
            eprintln!("Error: {}", message);
        }
    }
}

fn part1_solution(input: &Vec<String>) -> u128 {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    let mut distance_sums = 0;

    for line in input {
        let split: Vec<_> = line
            .split("   ")
            .map(|nbr| nbr.parse::<i128>().unwrap())
            .collect();
        println!("{split:?}");
        left_list.push(split[0]);
        right_list.push(split[1]);
    }

    left_list.sort();
    right_list.sort();
    println!("{left_list:?}");
    println!("{right_list:?}");

    for (left, right) in left_list.iter().zip(right_list.iter()) {
        distance_sums += (left - right).abs();
    }

    distance_sums as u128
}

fn part2_solution(input: &Vec<String>) -> u128 {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    let mut similarity_score = 0;

    for line in input {
        let split: Vec<_> = line
            .split("   ")
            .map(|nbr| nbr.parse::<u128>().unwrap())
            .collect();
        println!("{split:?}");
        left_list.push(split[0]);
        right_list.push(split[1]);
    }

    for num in left_list {
        let occurences = right_list.iter().filter(|nbr| **nbr == num).count();
        similarity_score += num * occurences as u128;
    }

    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// To check if modifications between different steps don't invalidate previous ones
    fn breaking_changes_test() {
        let input_path = "../inputs/day01/small.txt";
        let (_, input) = load_file(Some(input_path.to_string())).expect("Failed to load file");

        assert_eq!(part1_solution(&input), 11);

        let input_path = "../inputs/day01/small_2.txt";
        let (_, input) = load_file(Some(input_path.to_string())).expect("Failed to load file");
        assert_eq!(part2_solution(&input), 31);
    }
}
