use std::error::Error;
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
        total += determine_count(Some(&curr), Some(&prev), Some(&next), width);

        prev = curr;
        curr = next;
        next_line = lines.next().map(|l| l.chars().collect::<Vec<char>>());
    }

    total += determine_count(Some(&curr), Some(&prev), None, width);

    Ok(total)
}

fn determine_count(
    curr: Option<&Vec<char>>,
    prev: Option<&Vec<char>>,
    next: Option<&Vec<char>>,
    width: i32,
) -> i32 {
    let mut total = 0;

    if curr.is_none() {
        return 0;
    }

    //Check if curr is none
    let results = determine_numbers_with_index(curr.unwrap().into_iter());

    for result in results {
        let start = result.1 - 1;
        let end = result.1 + result.2;

        let current = curr.unwrap();
        match current.get(start) {}
    }
    //check start - 1 and stop +2
    //check start - 1 until stop + 1 on prev if exists
    //check start - 1 until stop + 1 on next if exists
    //add parsed number to total.
    //keep interating
    return total;
}

fn determine_numbers_with_index(into_iter: std::slice::Iter<'_, char>) -> Vec<(i32, i32, i32)> {
    //iterate over curr
    //skip points
    //take until non-digit and remember index
    todo!()
}

fn determine_width(input: &str) -> i32 {
    input
        .lines()
        .next()
        .unwrap_or_default()
        .len()
        .try_into()
        .unwrap()
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

    #[test]
    fn test_example1() {
        let sum = calculate_sum(EXAMPLE1);
        assert_eq!(sum.unwrap(), 4361);
    }
}
