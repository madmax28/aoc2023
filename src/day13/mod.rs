use std::{cmp::max, collections::HashSet, iter::repeat, str::FromStr};

struct Map {
    rocks: HashSet<(i32, i32)>,
    size: (i32, i32),
}

impl FromStr for Map {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        let mut size = (0i32, 0i32);
        let rocks = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| repeat(y).zip(l.chars().enumerate()))
            .filter(|(_, (_, c))| c == &'#')
            .map(|(y, (x, _))| {
                let (x, y) = (x as i32, y as i32);
                size.0 = max(size.0, x + 1);
                size.1 = max(size.1, y + 1);
                (x, y)
            })
            .collect();
        Ok(Map { rocks, size })
    }
}

impl Map {
    fn smudges_x(&self, x: i32) -> usize {
        self.rocks
            .iter()
            .filter(|(xx, yy)| {
                let x = xx + 2 * (x - xx) - 1;
                x >= 0 && x < self.size.0 && !self.rocks.contains(&(x, *yy))
            })
            .count()
    }

    fn smudges_y(&self, y: i32) -> usize {
        self.rocks
            .iter()
            .filter(|(xx, yy)| {
                let y = yy + 2 * (y - yy) - 1;
                y >= 0 && y < self.size.1 && !self.rocks.contains(&(*xx, y))
            })
            .count()
    }

    fn score(&self, smudge_count: usize) -> i32 {
        if let Some(x) = (1..self.size.0).find(|x| self.smudges_x(*x) == smudge_count) {
            return x;
        }

        if let Some(y) = (1..self.size.1).find(|y| self.smudges_y(*y) == smudge_count) {
            return 100 * y;
        }

        unreachable!()
    }
}

fn parse(input: &str) -> crate::Result<Vec<Map>> {
    let maps = input
        .split("\n\n")
        .map(Map::from_str)
        .collect::<Result<_, _>>()?;
    Ok(maps)
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let sum = parse(input)?.into_iter().map(|map| map.score(0)).sum();
    Ok(sum)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let sum = parse(input)?.into_iter().map(|map| map.score(1)).sum();
    Ok(sum)
}
