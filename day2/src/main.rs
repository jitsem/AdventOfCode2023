use std::error::Error;
const INPUT: &str = include_str!("./input.txt");

struct Game {
    id: i32,
    draws: Vec<Draw>,
}

struct Draw {
    r: i32,
    g: i32,
    b: i32,
}

impl Game {
    //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    fn from(input_line: &str) -> Result<Self, Box<dyn Error>> {
        let mut input = input_line.split([':', ';']);
        let id = input
            .next()
            .ok_or_else(|| "Wrong format")?
            .split_whitespace()
            .last()
            .ok_or_else(|| "Wrong format")?
            .parse()?;
        let draw_res: Result<Vec<Draw>, Box<dyn Error>> =
            input.map(|s| Draw::from(s.trim_start())).collect();
        let draws = draw_res?;
        Ok(Game { id, draws: draws })
    }
    fn max_red(&self) -> i32 {
        self.draws.iter().map(|d| d.r).max().unwrap_or_default()
    }

    fn max_green(&self) -> i32 {
        self.draws.iter().map(|d| d.g).max().unwrap_or_default()
    }

    fn max_blue(&self) -> i32 {
        self.draws.iter().map(|d| d.b).max().unwrap_or_default()
    }
}

impl Draw {
    //1 red, 2 green, 6 blue
    fn from(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for ele in input.split(", ") {
            let mut parts = ele.split_whitespace();
            let value = parts
                .next()
                .ok_or_else(|| "Invalid input format")?
                .parse()?;
            match parts.next() {
                Some("red") => r = value,
                Some("green") => g = value,
                Some("blue") => b = value,
                _ => return Err("Invalid color".into()),
            }
        }

        Ok(Draw { r, g, b })
    }
}

fn calculate_sum(input: &str) -> Result<i32, Box<dyn Error>> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    input
        .lines()
        .map(Game::from)
        .filter(|g| {
            g.as_ref().is_ok_and(|game| {
                game.max_red() <= max_red
                    && game.max_green() <= max_green
                    && game.max_blue() <= max_blue
            })
        })
        .try_fold(0, |acc, g| g.map(|n| n.id + acc))
}

fn main() {
    println!("Hello, day 2!");
    let one = calculate_sum(INPUT);
    println!("Part 1: {}", one.unwrap());
}

#[cfg(test)]
mod test_input {
    use super::*;
    const EXAMPLE1: &str = include_str!("./example1.txt");

    #[test]
    fn test_example1() {
        let sum = calculate_sum(EXAMPLE1);
        assert_eq!(sum.unwrap(), 8);
    }
}
