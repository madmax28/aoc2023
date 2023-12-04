use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug)]
struct Card {
    winners: Vec<u32>,
    numbers: Vec<u32>,
    count: u32,
}

impl Card {
    fn score(self) -> u32 {
        let mut score = 0;
        for num in &self.numbers {
            if self.winners.contains(num) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }

    fn score_p2(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winners.contains(n))
            .count()
    }
}

impl FromStr for Card {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        let parts = s
            .split_once(':')
            .ok_or(crate::Error::boxed(Error::InvalidInput))?
            .1
            .split_once('|')
            .ok_or(crate::Error::boxed(Error::InvalidInput))?;
        let winners: Vec<u32> = parts
            .0
            .split_whitespace()
            .map(str::trim)
            .filter_map(|s| s.parse().ok())
            .collect();
        let numbers: Vec<u32> = parts
            .1
            .split_whitespace()
            .map(str::trim)
            .filter_map(|s| s.parse().ok())
            .collect();
        let count = 1;
        Ok(Card {
            winners,
            numbers,
            count,
        })
    }
}

pub fn part1(input: &str) -> crate::Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        sum += line.parse::<Card>()?.score();
    }
    Ok(sum)
}

pub fn part2(input: &str) -> crate::Result<u32> {
    let mut cards = input
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    for index in 0..cards.len() {
        for i in index + 1..=index + cards[index].score_p2() {
            cards
                .get_mut(i)
                .ok_or(crate::Error::boxed(Error::InvalidInput))?
                .count += cards[index].count;
        }
    }
    Ok(cards.into_iter().map(|c| c.count).sum())
}
