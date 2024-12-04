use std::ops::Mul;

use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i32> {
    let multiply_regex = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let mut results: Vec<i32> = Vec::new();
    for (_, [first, second]) in multiply_regex.captures_iter(input).map(|c| c.extract()) {
        results.push(
            first
                .parse::<i32>()
                .unwrap()
                .mul(second.parse::<i32>().unwrap()),
        );
    }

    return Some(results.iter().sum::<i32>());
}

pub fn part_two(input: &str) -> Option<i32> {
    let function_regex = Regex::new(r"(mul|don't|do)\((.*?)\)").unwrap();

    let mut enabled: bool = true;
    let mut results: Vec<i32> = Vec::new();
    for (full, [_mutator, _rest]) in function_regex.captures_iter(input).map(|c| c.extract()) {
        if full.starts_with("don't") {
            enabled = false;
            continue;
        }

        if full.starts_with("do") {
            enabled = true;
            continue;
        }

        if full.starts_with("mul") && enabled {
            let multiply_regex = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
            let captured = multiply_regex.captures(full).map(|c| c.extract());

            if captured.is_some() {
                let (_, [first, second]) = captured.unwrap();

                results.push(
                    first
                        .parse::<i32>()
                        .unwrap()
                        .mul(second.parse::<i32>().unwrap()),
                );
            }
        }
    }

    return Some(results.iter().sum::<i32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
