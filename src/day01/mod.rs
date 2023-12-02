#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn digit_p1(cand: &str) -> Option<u32> {
    cand.chars().next().and_then(|c| c.to_digit(10))
}

const DIGITS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn digit_p2(cand: &str) -> Option<u32> {
    if let Some(&d) = DIGITS.iter().find(|&d| cand.starts_with(d.0)) {
        Some(d.1)
    } else {
        digit_p1(cand)
    }
}

fn calibration_value(line: &str, digit: fn(&str) -> Option<u32>) -> crate::Result<u32> {
    let first = (0..line.len())
        .find_map(|needle| digit(&line[needle..]))
        .ok_or(crate::Error::boxed(Error::InvalidInput))?;
    let last = (0..line.len())
        .rev()
        .find_map(|needle| digit(&line[needle..]))
        .ok_or(crate::Error::boxed(Error::InvalidInput))?;
    Ok(first * 10 + last)
}

pub fn sum(input: &str, digit: fn(&str) -> Option<u32>) -> crate::Result<u32> {
    Ok(input
        .lines()
        .map(|line| calibration_value(line, digit))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum())
}

pub fn part1(input: &str) -> crate::Result<u32> {
    sum(input, digit_p1)
}

pub fn part2(input: &str) -> crate::Result<u32> {
    sum(input, digit_p2)
}
