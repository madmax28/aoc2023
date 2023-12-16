use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam {
    pos: (i32, i32),
    dir: Dir,
}

impl Beam {
    fn new(pos: (i32, i32), dir: Dir) -> Self {
        Beam { pos, dir }
    }

    fn mv(&mut self) {
        match self.dir {
            Dir::North => self.pos.1 -= 1,
            Dir::East => self.pos.0 += 1,
            Dir::South => self.pos.1 += 1,
            Dir::West => self.pos.0 -= 1,
        }
    }

    fn reflect(self, mirror: char) -> Vec<Self> {
        let dirs = match (&self.dir, mirror) {
            (Dir::North, '/') => vec![Dir::East],
            (Dir::North, '\\') => vec![Dir::West],
            (Dir::North, '-') => vec![Dir::East, Dir::West],
            (Dir::East, '/') => vec![Dir::North],
            (Dir::East, '\\') => vec![Dir::South],
            (Dir::East, '|') => vec![Dir::North, Dir::South],
            (Dir::South, '/') => vec![Dir::West],
            (Dir::South, '\\') => vec![Dir::East],
            (Dir::South, '-') => vec![Dir::East, Dir::West],
            (Dir::West, '/') => vec![Dir::South],
            (Dir::West, '\\') => vec![Dir::North],
            (Dir::West, '|') => vec![Dir::North, Dir::South],
            _ => vec![self.dir],
        };
        dirs.into_iter().map(|d| Self::new(self.pos, d)).collect()
    }
}

fn parse(input: &str) -> (HashMap<(i32, i32), char>, (i32, i32)) {
    let mut sz = (0, 0);
    let mut map = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            sz.0 = sz.0.max(x as i32 + 1);
            sz.1 = sz.1.max(y as i32 + 1);
            map.insert((x as i32, y as i32), c);
        }
    }
    (map, sz)
}

fn shine(beam: Beam, map: &HashMap<(i32, i32), char>) -> usize {
    let mut beams = vec![beam];
    let mut energized = HashSet::new();
    let mut seen = HashSet::new();
    while let Some(mut beam) = beams.pop() {
        if !seen.insert(beam.clone()) {
            continue;
        }

        beam.mv();
        if let Some(c) = map.get(&beam.pos) {
            energized.insert(beam.pos);
            beams.extend(beam.reflect(*c));
        }
    }
    energized.len()
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let (map, _) = parse(input);
    Ok(shine(Beam::new((-1, 0), Dir::East), &map))
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let (map, sz) = parse(input);
    let mut max = 0;
    for x in 0..sz.0 {
        max = max.max(shine(Beam::new((x, -1), Dir::South), &map));
        max = max.max(shine(Beam::new((x, sz.1), Dir::North), &map));
    }
    for y in 0..sz.1 {
        max = max.max(shine(Beam::new((-1, y), Dir::East), &map));
        max = max.max(shine(Beam::new((sz.0, y), Dir::West), &map));
    }
    Ok(max)
}
