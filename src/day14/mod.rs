use std::{collections::HashMap, str::FromStr};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Map {
    map: Vec<Vec<char>>,
}

impl FromStr for Map {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        let map = s.lines().map(|line| line.chars().collect()).collect();
        Ok(Map { map })
    }
}

impl Map {
    fn mv(&mut self, direction: (i32, i32)) -> bool {
        let mut moved = false;
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let from = (x as i32, y as i32);
                let to = (x as i32 + direction.0, y as i32 + direction.1);
                if matches!(self.get(from), Some('O')) && matches!(self.get(to), Some('.')) {
                    moved = true;
                    self.map[from.1 as usize][from.0 as usize] = '.';
                    self.map[to.1 as usize][to.0 as usize] = 'O';
                }
            }
        }
        moved
    }

    fn get(&self, pos: (i32, i32)) -> Option<char> {
        if pos.0 >= 0 && pos.1 >= 0 {
            self.map
                .get(pos.1 as usize)
                .and_then(|ys| ys.get(pos.0 as usize))
                .copied()
        } else {
            None
        }
    }

    fn cycle(&mut self) {
        while self.mv((0, -1)) {}
        while self.mv((-1, 0)) {}
        while self.mv((0, 1)) {}
        while self.mv((1, 0)) {}
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == 'O' {
                    score += self.map.len() - y;
                }
            }
        }
        score
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let mut map: Map = input.parse()?;
    while map.mv((0, -1)) {}
    Ok(map.score())
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let mut map: Map = input.parse()?;

    const TGT: usize = 1000000000;
    let mut cycle = 0;
    let mut seen = HashMap::new();
    seen.insert(map.clone(), cycle);
    while cycle < TGT {
        map.cycle();
        cycle += 1;

        if let Some(first) = seen.get(&map) {
            let len = cycle - first;
            let skip = (TGT - cycle) / len;
            cycle += skip * len;
        }
        seen.insert(map.clone(), cycle);
    }
    Ok(map.score())
}
