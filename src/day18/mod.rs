#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn mv(&self, pos: (i64, i64), steps: i64) -> (i64, i64) {
        match self {
            Direction::North => (pos.0, pos.1 - steps),
            Direction::East => (pos.0 + steps, pos.1),
            Direction::South => (pos.0, pos.1 + steps),
            Direction::West => (pos.0 - steps, pos.1),
        }
    }
}

fn parse(input: &str) -> crate::Result<Vec<(Direction, i64)>> {
    let mut res = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let dir = match parts
            .next()
            .ok_or(crate::Error::boxed(Error::InvalidInput))?
        {
            "U" => Direction::North,
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            _ => return Err(crate::Error::boxed(Error::InvalidInput)),
        };
        let steps = parts
            .next()
            .ok_or(crate::Error::boxed(Error::InvalidInput))?
            .parse()?;
        res.push((dir, steps));
    }
    Ok(res)
}

fn shoelace(vertices: &[(i64, i64)]) -> i64 {
    let mut area = 0;
    let n = vertices.len() - 1;
    for idx in 0..n {
        area += vertices[idx].0 * vertices[idx + 1].1;
        area -= vertices[idx + 1].0 * vertices[idx].1;
    }
    area += vertices[n].0 * vertices[0].1;
    area -= vertices[0].0 * vertices[n].1;
    area / 2
}

fn solve(instr: &[(Direction, i64)]) -> i64 {
    let (mut x, mut y) = (0i64, 0i64);
    let vertices: Vec<_> = instr
        .iter()
        .map(|(dir, steps)| {
            (x, y) = dir.mv((x, y), *steps);
            (x, y)
        })
        .collect();
    let area = shoelace(&vertices);
    let border: i64 = instr.iter().map(|(_, steps)| steps).sum();
    area + border / 2 + 1
}

pub fn part1(input: &str) -> crate::Result<i64> {
    Ok(solve(&parse(input)?))
}

fn parse_p2(input: &str) -> crate::Result<Vec<(Direction, i64)>> {
    let mut res = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let hex = parts
            .nth(2)
            .ok_or(crate::Error::boxed(Error::InvalidInput))?;
        let dir = match hex
            .chars()
            .nth(7)
            .ok_or(crate::Error::boxed(Error::InvalidInput))?
        {
            '0' => Direction::East,
            '1' => Direction::South,
            '2' => Direction::West,
            '3' => Direction::North,
            _ => return Err(crate::Error::boxed(Error::InvalidInput)),
        };
        let steps = i64::from_str_radix(&hex[2..7], 16)?;
        res.push((dir, steps));
    }
    Ok(res)
}

pub fn part2(input: &str) -> crate::Result<i64> {
    Ok(solve(&parse_p2(input)?))
}
