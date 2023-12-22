use std::{
    collections::{HashMap, HashSet},
    iter,
};

type Vec3 = (i32, i32, i32);

#[derive(Debug, Clone)]
struct Line {
    from: Vec3,
    to: Vec3,
}

impl Line {
    fn new(from: Vec3, to: Vec3) -> Self {
        Line { from, to }
    }

    fn points(&self) -> impl Iterator<Item = Vec3> + '_ {
        (self.from.0..=self.to.0)
            .flat_map(|x| iter::repeat(x).zip(self.from.1..=self.to.1))
            .flat_map(|xy| iter::repeat(xy).zip(self.from.2..=self.to.2))
            .map(|((x, y), z)| (x, y, z))
    }

    fn adjacent_z(&self, offset: i32) -> Line {
        if self.from.2 != self.to.2 {
            // vertical
            let z = if offset > 0 {
                self.to.2 + offset
            } else {
                self.from.2 + offset
            };
            let p = (self.from.0, self.from.1, z);
            Line::new(p, p)
        } else {
            // horizontal
            let from = (self.from.0, self.from.1, self.from.2 + offset);
            let to = (self.to.0, self.to.1, self.to.2 + offset);
            Line::new(from, to)
        }
    }

    fn fall(&mut self) {
        self.from.2 -= 1;
        self.to.2 -= 1;
    }
}

#[derive(Debug, Clone)]
struct Bricks {
    bricks: Vec<Line>,
    lookup: HashMap<Vec3, usize>,
}

impl Bricks {
    fn new(bricks: Vec<Line>) -> Self {
        let mut lookup = HashMap::new();
        for (idx, brick) in bricks.iter().enumerate() {
            for p in brick.points() {
                lookup.insert(p, idx);
            }
        }
        Bricks { bricks, lookup }
    }

    fn fall(&mut self) -> HashSet<usize> {
        let mut fallen = HashSet::new();
        let mut did_move = true;
        while did_move {
            did_move = false;

            for (idx, brick) in self.bricks.iter_mut().enumerate() {
                for p in brick.points() {
                    self.lookup.remove(&p);
                }

                while brick
                    .adjacent_z(-1)
                    .points()
                    .all(|p| p.2 > 0 && !self.lookup.contains_key(&p))
                {
                    did_move = true;
                    fallen.insert(idx);
                    brick.fall();
                }

                for p in brick.points() {
                    self.lookup.insert(p, idx);
                }
            }
        }
        fallen
    }

    fn adjacent_z(&self, idx: usize, offset: i32) -> HashSet<usize> {
        self.bricks[idx]
            .adjacent_z(offset)
            .points()
            .filter_map(|p| self.lookup.get(&p))
            .cloned()
            .collect::<HashSet<usize>>()
    }

    fn count_supported(&self, idx: usize) -> usize {
        let mut supports: Vec<HashSet<usize>> = (0..self.bricks.len())
            .map(|brick| self.adjacent_z(brick, -1))
            .collect();
        for support in &mut supports {
            support.retain(|i| i != &idx);
        }

        let mut cnt = 0;
        let mut supported = self.adjacent_z(idx, 1);
        supported.retain(|idx| supports[*idx].is_empty());
        while !supported.is_empty() {
            cnt += supported.len();

            for idx in &supported {
                for support in &mut supports {
                    support.retain(|i| i != idx);
                }
            }

            supported = supported
                .into_iter()
                .flat_map(|idx| self.adjacent_z(idx, 1))
                .filter(|idx| supports[*idx].is_empty())
                .collect();
        }
        cnt
    }
}

fn parse(input: &str) -> Bricks {
    let mut bricks = Vec::new();
    for line in input.lines() {
        let (from, to) = line.split_once('~').unwrap();
        let mut from = from.split(',').map(|n| n.parse::<i32>().unwrap());
        let from = (
            from.next().unwrap(),
            from.next().unwrap(),
            from.next().unwrap(),
        );
        let mut to = to.split(',').map(|n| n.parse::<i32>().unwrap());
        let to = (to.next().unwrap(), to.next().unwrap(), to.next().unwrap());
        bricks.push(Line::new(from, to));
    }
    Bricks::new(bricks)
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let mut bricks = parse(input);
    bricks.fall();
    let disintegratable = (0..bricks.bricks.len())
        .filter(|idx| bricks.count_supported(*idx) == 0)
        .count();
    Ok(disintegratable)
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let mut bricks = parse(input);
    bricks.fall();
    let sum = (0..bricks.bricks.len())
        .map(|idx| bricks.count_supported(idx))
        .sum();
    Ok(sum)
}
