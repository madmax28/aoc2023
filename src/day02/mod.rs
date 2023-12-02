use std::cmp::max;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug)]
struct CubeCount {
    r: u32,
    g: u32,
    b: u32,
}

impl CubeCount {
    fn max(self, rhs: &CubeCount) -> CubeCount {
        CubeCount {
            r: max(self.r, rhs.r),
            g: max(self.g, rhs.g),
            b: max(self.b, rhs.b),
        }
    }
}

impl FromStr for CubeCount {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        let [mut r, mut g, mut b] = [0, 0, 0];
        for color in s.trim().split(", ") {
            let mut parts = color.split(' ');
            let cnt = parts
                .next()
                .ok_or(crate::Error::boxed(Error::InvalidInput))?
                .parse::<u32>()?;
            match parts
                .next()
                .ok_or(crate::Error::boxed(Error::InvalidInput))?
            {
                "red" => r += cnt,
                "green" => g += cnt,
                "blue" => b += cnt,
                _ => return Err(crate::Error::boxed(Error::InvalidInput)),
            }
        }
        Ok(CubeCount { r, g, b })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<CubeCount>,
}

impl Game {
    fn is_possible(&self, cube_count: &CubeCount) -> bool {
        self.rounds
            .iter()
            .all(|cc| cc.r <= cube_count.r && cc.g <= cube_count.g && cc.b <= cube_count.b)
    }

    fn power(&self) -> u32 {
        let cnt = self
            .rounds
            .iter()
            .fold(CubeCount { r: 0, g: 0, b: 0 }, CubeCount::max);
        cnt.r * cnt.g * cnt.b
    }
}

impl FromStr for Game {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        let mut parts = s.split(':');
        let id = parts
            .next()
            .ok_or(crate::Error::boxed(Error::InvalidInput))?
            .trim_matches(|c: char| !c.is_numeric())
            .parse::<u32>()?;
        let rounds = parts
            .next()
            .ok_or(crate::Error::boxed(Error::InvalidInput))?
            .split(';')
            .map(CubeCount::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Game { id, rounds })
    }
}

pub fn part1(input: &str) -> crate::Result<u32> {
    let cube_count = CubeCount {
        r: 12,
        g: 13,
        b: 14,
    };
    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(games
        .iter()
        .filter_map(|g| {
            if g.is_possible(&cube_count) {
                Some(g.id)
            } else {
                None
            }
        })
        .sum())
}

pub fn part2(input: &str) -> crate::Result<u32> {
    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(games.iter().map(Game::power).sum())
}
