use std::{
    collections::{BTreeMap, HashMap, HashSet},
    iter,
};

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn mv(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (pos.0, pos.1 - 1),
            Direction::East => (pos.0 + 1, pos.1),
            Direction::South => (pos.0, pos.1 + 1),
            Direction::West => (pos.0 - 1, pos.1),
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

type Map = HashMap<(i32, i32), u32>;

fn parse(input: &str) -> crate::Result<(Map, (i32, i32))> {
    let mut sz = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| iter::repeat(y).zip(l.chars().enumerate()))
        .map(|(y, (x, c))| {
            sz.0 = sz.0.max(1 + x as i32);
            sz.1 = sz.1.max(1 + y as i32);
            if let Some(n) = c.to_digit(10) {
                Ok(((x as i32, y as i32), n))
            } else {
                Err(crate::Error::boxed(Error::InvalidInput))
            }
        })
        .collect::<Result<_, _>>()?;
    Ok((map, sz))
}

type State = ((i32, i32), Option<(Direction, u32)>);
type Frontier = BTreeMap<u32, Vec<State>>;
type Seen = HashSet<State>;

fn search(map: &Map, tgt: (i32, i32), min_straight: u32, max_straight: u32) -> u32 {
    let mut frontier = Frontier::new();
    frontier.insert(0, vec![((0, 0), None)]);
    let mut seen = Seen::new();
    loop {
        let (cost, mut states) = frontier.pop_first().unwrap();
        let (pos, path) = states.pop().unwrap();
        if !states.is_empty() {
            frontier.insert(cost, states);
        }

        if pos == tgt {
            return cost;
        }

        for new_dir in DIRS {
            let new_straight = if let Some((dir, straight)) = path {
                if dir.reverse() == new_dir {
                    continue;
                }

                if dir == new_dir && straight == max_straight {
                    continue;
                }

                if dir != new_dir && straight < min_straight {
                    continue;
                }

                if dir == new_dir {
                    1 + straight
                } else {
                    1
                }
            } else {
                1
            };

            let new_pos = new_dir.mv(pos);
            if let Some(tile_cost) = map.get(&new_pos) {
                let new_cost = cost + tile_cost;
                let new_state = (new_pos, Some((new_dir, new_straight)));
                if !seen.insert(new_state) {
                    continue;
                }

                frontier.entry(new_cost).or_default().push(new_state);
            }
        }
    }
}

pub fn part1(input: &str) -> crate::Result<u32> {
    let (map, sz) = parse(input)?;
    Ok(search(&map, (sz.0 - 1, sz.1 - 1), 0, 3))
}

pub fn part2(input: &str) -> crate::Result<u32> {
    let (map, sz) = parse(input)?;
    Ok(search(&map, (sz.0 - 1, sz.1 - 1), 4, 10))
}
