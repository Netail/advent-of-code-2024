advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        let split = line.split_whitespace();

        for (i, split_item) in split.enumerate() {
            if i == 0 {
                left.push(split_item.parse::<i32>().unwrap())
            } else {
                right.push(split_item.parse::<i32>().unwrap())
            };
        }
    }

    left.sort();
    right.sort();

    let mut answer: Vec<i32> = Vec::new();

    for (i, item) in left.iter().enumerate() {
        answer.push((item - right.get(i).unwrap_or(&0)).abs());
    }

    return Some(answer.iter().sum::<i32>());
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        let split = line.split_whitespace();

        for (i, split_item) in split.enumerate() {
            if i == 0 {
                left.push(split_item.parse::<i32>().unwrap())
            } else {
                right.push(split_item.parse::<i32>().unwrap())
            };
        }
    }

    let mut answer: Vec<i32> = Vec::new();

    for item in left {
        answer.push(item * (right.iter().filter(|&n| *n == item).count() as i32));
    }

    return Some(answer.iter().sum::<i32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
