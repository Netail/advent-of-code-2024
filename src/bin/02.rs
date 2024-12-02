advent_of_code::solution!(2);

fn iterate_over_array(levels: Vec<i32>) -> bool {
    let mut increasing: Option<bool> = None;
    let mut safe: bool = true;

    levels.into_iter().reduce(|prev, curr| {
        let diff = prev - curr;
        let mut increasing_level: bool = false;

        if diff > 0 {
            increasing_level = true;
        } else if diff < 0 {
            increasing_level = false;
        }

        if increasing.is_none() {
            increasing = Some(increasing_level);
        } else if increasing != Some(increasing_level) {
            safe = false;
        }

        if diff.abs() == 0 || diff.abs() > 3 {
            safe = false;
        }

        return curr;
    });

    return safe;
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut safe_count: i32 = 0;

    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|i| i32::from_str_radix(i, 10))
            .map(Result::unwrap)
            .collect();

        let safe: bool = iterate_over_array(levels);

        if safe {
            safe_count += 1;
        }
    }

    return Some(safe_count);
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut safe_count: i32 = 0;

    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|i| i32::from_str_radix(i, 10))
            .map(Result::unwrap)
            .collect();

        let mut safe: bool = iterate_over_array(levels.clone());

        if safe {
            safe_count += 1;
            continue;
        }

        for (i, f) in levels.clone().into_iter().enumerate() {
            let mut copy: Vec<i32> = levels.clone();
            copy.remove(i);

            safe = iterate_over_array(copy);
            if safe {
                break;
            }
        }

        if safe {
            safe_count += 1;
        }
    }

    return Some(safe_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
