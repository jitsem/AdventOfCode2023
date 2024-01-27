use std::{error::Error, num::ParseIntError};
const INPUT: &str = include_str!("./input.txt");

fn calculate_sum(input: &str) -> Result<i32, Box<dyn Error>> {
    let width = determine_width(input);
    let mut lines = input.lines().peekable();
    let mut total = 0;

    if lines.peek().is_none() {
        return Err("Expected at least some input".into());
    }
    let mut prev = Vec::new();
    let mut curr = lines.next().unwrap().chars().collect::<Vec<char>>();
    let mut next_line = lines.next().map(|l| l.chars().collect::<Vec<char>>());

    while let Some(next) = next_line {
        total += determine_count(
            Some(&curr),
            Some(&prev),
            Some(&next),
            width.try_into().unwrap(),
        )?;

        prev = curr;
        curr = next;
        next_line = lines.next().map(|l| l.chars().collect::<Vec<char>>());
    }
    total += determine_count(Some(&curr), Some(&prev), None, width.try_into().unwrap())?;
    Ok(total)
}

fn determine_count(
    curr: Option<&Vec<char>>,
    prev: Option<&Vec<char>>,
    next: Option<&Vec<char>>,
    width: usize,
) -> Result<i32, Box<dyn Error>> {
    let mut total = 0;

    if let Some(current) = curr {
        let results = determine_numbers_with_index(current);

        for (count, start_index, length) in results? {
            let end = std::cmp::min(start_index + length, width - 1);
            let start = start_index.saturating_sub(1);
            let check_match = |data: Option<&Vec<char>>| {
                data.and_then(|vec| vec.get(start..=end))
                    .map_or(false, |s| {
                        s.iter().any(|&c| !c.is_ascii_digit() && c != '.')
                    })
            };

            if check_match(Some(current)) || check_match(prev) || check_match(next) {
                total += count;
            }
        }
    }
    Ok(total)
}

fn determine_numbers_with_index(slice: &[char]) -> Result<Vec<(i32, usize, usize)>, ParseIntError> {
    let mut results = Vec::new();
    let mut start_index = 0;

    while let Some(end_index) = slice[start_index..]
        .iter()
        .position(|c| !c.is_ascii_digit())
    {
        if end_index > 0 {
            let number_str: String = slice[start_index..start_index + end_index].iter().collect();
            let number: i32 = number_str.parse()?;
            results.push((number, start_index, end_index));
        }
        start_index += end_index + 1;
    }

    if start_index < slice.len() {
        let number_str: String = slice[start_index..].iter().collect();
        let number: i32 = number_str.parse()?;
        results.push((number, start_index, slice.len()));
    }

    Ok(results)
}

fn determine_width(input: &str) -> i32 {
    input
        .lines()
        .next()
        .unwrap_or_default()
        .len()
        .try_into()
        .unwrap_or_default()
}

fn main() {
    println!("Hello, day 2!");
    let one = calculate_sum(INPUT);
    println!("Part 1: {}", one.unwrap());
    //let two = calculate_power(INPUT);
    //println!("Part 2: {}", two.unwrap());
}

#[cfg(test)]
mod test_input {
    use super::*;
    const EXAMPLE1: &str = include_str!("./example1.txt");
    const EXAMPLE2: &str = include_str!("./example2.txt");

    #[test]
    fn test_example1() {
        let sum = calculate_sum(EXAMPLE1);
        assert_eq!(sum.unwrap(), 4361);
    }
    #[test]
    fn test_example2() {
        let sum = calculate_sum(EXAMPLE2);
        assert_eq!(sum.unwrap(), 4382);
    }
}
