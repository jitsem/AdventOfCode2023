const INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("Hello, day 1!");
    let one = calculate_sum(INPUT);
    println!("Part 1: {}", one.unwrap());
    let two = calculate_sum_with_written(INPUT);
    println!("Part 2: {}", two.unwrap())
}

fn calculate_sum(input: &str) -> Result<i32, std::num::ParseIntError> {
    input
        .lines()
        .filter_map(|line| {
            let mut digits = line.chars().filter(|c| c.is_ascii_digit());
            match (digits.next(), digits.last()) {
                (Some(first), Some(last)) => Some(format!("{}{}", first, last)),
                (Some(single_digit), None) => Some(format!("{}{}", single_digit, single_digit)),
                _ => None,
            }
        })
        .map(|nr_string| nr_string.parse::<i32>())
        .try_fold(0, |acc, num| num.map(|n| acc + n))
}

fn calculate_sum_with_written(input: &str) -> Result<i32, std::num::ParseIntError> {
    let mut result = input.to_lowercase();

    let replacements = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];
    for (from, to) in replacements {
        result = result.replace(from, to);
    }
    calculate_sum(result.as_str())
}

#[cfg(test)]
mod test_input {
    use crate::calculate_sum;
    use crate::calculate_sum_with_written;
    const EXAMPLE1: &str = include_str!("./example1.txt");
    const EXAMPLE2: &str = include_str!("./example2.txt");

    #[test]
    fn test_example1() {
        let sum = calculate_sum(EXAMPLE1);
        assert_eq!(sum.unwrap(), 142);
    }

    #[test]
    fn test_example2() {
        let sum = calculate_sum_with_written(EXAMPLE2);
        assert_eq!(sum.unwrap(), 281);
    }
}
