use std::collections::HashSet;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn card_value(card: char, is_p2: bool) -> crate::Result<usize> {
    let val = match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => {
            if is_p2 {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => return Err(crate::Error::boxed(Error::InvalidInput)),
    };
    Ok(val)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    of_a_kind: usize,
    extra_pairs: usize,
    cards: Vec<usize>,
    bid: usize,
}

impl Hand {
    fn new(cards: Vec<usize>, bid: usize) -> Self {
        let of_a_kind = cards
            .iter()
            .map(|c| cards.iter().filter(|cc| c == *cc || &1 == *cc).count())
            .max()
            .unwrap_or(0);
        let num_pairs = cards
            .iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .filter(|c| *c != &1)
            .filter(|c| cards.iter().filter(|cc| c == cc).count() > 1)
            .count();
        Hand {
            of_a_kind,
            extra_pairs: if num_pairs > 1 { 1 } else { 0 },
            cards,
            bid,
        }
    }

    fn from_str(s: &str, is_p2: bool) -> crate::Result<Self> {
        let mut parts = s.split_whitespace();
        let cards = parts
            .next()
            .ok_or(crate::Error::boxed(Error::InvalidInput))?
            .chars()
            .map(|c| card_value(c, is_p2))
            .collect::<Result<_, _>>()?;
        let bid = parts
            .next()
            .ok_or(crate::Error::boxed(Error::InvalidInput))?
            .parse()?;
        Ok(Hand::new(cards, bid))
    }
}

fn solve(input: &str, is_p2: bool) -> crate::Result<usize> {
    let mut hands = input
        .lines()
        .map(|l| Hand::from_str(l, is_p2))
        .collect::<Result<Vec<_>, _>>()?;
    hands.sort();
    let winnings = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (1 + idx) * hand.bid)
        .sum();
    Ok(winnings)
}

pub fn part1(input: &str) -> crate::Result<usize> {
    solve(input, false)
}

pub fn part2(input: &str) -> crate::Result<usize> {
    solve(input, true)
}
