use std::{
    collections::{HashMap, HashSet},
    iter,
    str::FromStr,
};

type Point = (i32, i32);
type Edges = HashMap<Point, HashSet<(Point, usize)>>;

struct Map {
    start: Point,
    end: Point,
    tiles: HashMap<Point, char>,
}

impl Map {
    fn neighbors(&self, pos: Point) -> impl Iterator<Item = Point> + '_ {
        match self.tiles.get(&pos).unwrap() {
            '.' => vec![
                (pos.0 + 1, pos.1),
                (pos.0 - 1, pos.1),
                (pos.0, pos.1 + 1),
                (pos.0, pos.1 - 1),
            ],
            '>' => vec![(pos.0 + 1, pos.1)],
            'v' => vec![(pos.0, pos.1 + 1)],
            _ => unreachable!(),
        }
        .into_iter()
        .filter(|p| !matches!(self.tiles.get(p), None | Some('#')))
    }

    fn find_paths(
        &self,
        path: &mut Vec<Point>,
        visited: &mut HashSet<Point>,
        solved: &mut Vec<usize>,
    ) {
        let pos = *path.last().unwrap();
        if pos == self.end {
            solved.push(path.len() - 1);
            return;
        }

        for neighbor in self.neighbors(pos) {
            if visited.insert(neighbor) {
                path.push(neighbor);
                self.find_paths(path, visited, solved);
                path.pop();
                visited.remove(&neighbor);
            }
        }
    }

    fn is_intersection(&self, p: Point) -> bool {
        p == self.start
            || p == self.end
            || [
                (p.0 + 1, p.1),
                (p.0 - 1, p.1),
                (p.0, p.1 + 1),
                (p.0, p.1 - 1),
            ]
            .into_iter()
            .all(|p| !matches!(self.tiles.get(&p), Some('.')))
    }

    fn find_edges(&self, mut path: Vec<Point>, edges: &mut Edges) {
        let pos = *path.last().unwrap();
        if path.len() > 1 && self.is_intersection(pos) {
            let start = *path.first().unwrap();
            let len = path.len() - 1;
            if !edges.entry(start).or_default().insert((pos, len)) {
                return;
            }
            edges.entry(pos).or_default().insert((start, len));
            path = vec![pos];
        }

        for neighbor in [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ] {
            if matches!(self.tiles.get(&neighbor), None | Some('#')) {
                continue;
            }

            if !path.contains(&neighbor) {
                path.push(neighbor);
                self.find_edges(path.clone(), edges);
                path.pop();
            }
        }
    }

    fn find_paths_p2(
        &self,
        path: &mut Vec<Point>,
        steps: usize,
        solved: &mut Vec<usize>,
        edges: &Edges,
    ) {
        let pos = *path.last().unwrap();
        if pos == self.end {
            solved.push(steps);
            return;
        }

        for (neighbor, dist) in &edges[&pos] {
            if path.contains(neighbor) {
                continue;
            }

            path.push(*neighbor);
            self.find_paths_p2(path, steps + dist, solved, edges);
            path.pop();
        }
    }
}

impl FromStr for Map {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let tiles = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| iter::repeat(y).zip(l.chars().enumerate()))
            .map(|(y, (x, c))| {
                let (x, y) = (x as i32, y as i32);
                if c == '.' {
                    if y == 0 {
                        start = (x, y);
                    }
                    end = (x, y);
                }
                ((x, y), c)
            })
            .collect();
        Ok(Map { start, end, tiles })
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let map: Map = input.parse()?;
    let mut solved = Vec::new();
    map.find_paths(&mut vec![map.start], &mut HashSet::new(), &mut solved);
    Ok(*solved.iter().max().unwrap())
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let map: Map = input.parse()?;
    let mut edges = HashMap::new();
    map.find_edges(vec![map.start], &mut edges);
    let mut solved = Vec::new();
    map.find_paths_p2(&mut vec![map.start], 0, &mut solved, &edges);
    Ok(*solved.iter().max().unwrap())
}
