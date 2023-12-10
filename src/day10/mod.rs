use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Debug)]
struct Tile {
    content: char,
    is_loop: bool,
    is_outside: [bool; 4], // NESW
}

impl Tile {
    fn new(content: char) -> Self {
        Tile {
            content,
            is_loop: false,
            is_outside: [false; 4],
        }
    }

    fn connections(&self, pos: (i32, i32)) -> Vec<(i32, i32)> {
        let (x, y) = pos;
        match self.content {
            '-' => vec![(x - 1, y), (x + 1, y)],
            '|' => vec![(x, y - 1), (x, y + 1)],
            'L' => vec![(x, y - 1), (x + 1, y)],
            'F' => vec![(x + 1, y), (x, y + 1)],
            '7' => vec![(x, y + 1), (x - 1, y)],
            'J' => vec![(x - 1, y), (x, y - 1)],
            _ => vec![],
        }
    }

    fn neighbors(&self, pos: (i32, i32)) -> Vec<((i32, i32), Dir)> {
        let mut res = Vec::new();
        let (x, y) = pos;
        if self.is_outside[0] {
            res.push(((x, y - 1), Dir::S));
            res.push(((x + 1, y), Dir::N));
            res.push(((x - 1, y), Dir::N));
        }
        if self.is_outside[1] {
            res.push(((x + 1, y), Dir::W));
            res.push(((x, y + 1), Dir::E));
            res.push(((x, y - 1), Dir::E));
        }
        if self.is_outside[2] {
            res.push(((x, y + 1), Dir::N));
            res.push(((x + 1, y), Dir::S));
            res.push(((x - 1, y), Dir::S));
        }
        if self.is_outside[3] {
            res.push(((x - 1, y), Dir::E));
            res.push(((x, y + 1), Dir::W));
            res.push(((x, y - 1), Dir::W));
        }
        res
    }

    fn fill_from(&mut self, dir: Dir) {
        if !self.is_loop {
            self.is_outside = [true; 4];
            return;
        }

        match (self.content, dir) {
            ('-', Dir::N) => self.is_outside = [true, false, false, false],
            ('-', Dir::S) => self.is_outside = [false, false, true, false],
            ('|', Dir::E) => self.is_outside = [false, true, false, false],
            ('|', Dir::W) => self.is_outside = [false, false, false, true],
            ('L', Dir::S) => self.is_outside = [false, false, true, true],
            ('L', Dir::W) => self.is_outside = [false, false, true, true],
            ('F', Dir::N) => self.is_outside = [true, false, false, true],
            ('F', Dir::W) => self.is_outside = [true, false, false, true],
            ('7', Dir::N) => self.is_outside = [true, true, false, false],
            ('7', Dir::E) => self.is_outside = [true, true, false, false],
            ('J', Dir::E) => self.is_outside = [false, true, true, false],
            ('J', Dir::S) => self.is_outside = [false, true, true, false],
            _ => (),
        }
    }
}

type Map = HashMap<(i32, i32), Tile>;

fn find_loop(start: (i32, i32), map: &mut Map) -> crate::Result<HashSet<(i32, i32)>> {
    let mut seen = HashSet::new();
    let mut to_visit = vec![start];
    while let Some(pos) = to_visit.pop() {
        map.get_mut(&pos).unwrap().is_loop = true;
        if seen.insert(pos) {
            to_visit.extend(map.get(&pos).unwrap().connections(pos));
        }
    }
    Ok(seen)
}

fn parse(input: &str) -> crate::Result<((i32, i32), Map)> {
    let mut map: Map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| repeat(y).zip(line.chars().enumerate()))
        .map(|(y, (x, c))| ((x as i32, y as i32), Tile::new(c)))
        .collect();
    let (x, y) = *map
        .iter()
        .find(|(_, tile)| tile.content == 'S')
        .map(|(pos, _)| pos)
        .ok_or(crate::Error::boxed(Error::InvalidInput))?;

    // fix starting location
    let connections = [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
        .into_iter()
        .map(|pos| {
            map.get(&pos)
                .map(|tile| tile.connections(pos).contains(&(x, y)))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    map.get_mut(&(x, y)).unwrap().content = match connections[..4] {
        [true, true, false, false] => 'L',
        [false, true, true, false] => 'F',
        [false, false, true, true] => '7',
        [true, false, false, true] => 'J',
        _ => return Err(crate::Error::boxed(Error::InvalidInput)),
    };

    Ok(((x, y), map))
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let (start, mut map) = parse(input)?;
    Ok(find_loop(start, &mut map)?.len() / 2)
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let (start, mut map) = parse(input)?;
    find_loop(start, &mut map)?;

    map.get_mut(&(0, 0)).unwrap().fill_from(Dir::N);
    let mut to_visit = vec![((0, 0), Dir::N)];
    let mut seen = HashSet::new();
    while let Some((pos, dir)) = to_visit.pop() {
        if seen.insert((pos, dir)) {
            if let Some(tile) = map.get_mut(&pos) {
                tile.fill_from(dir);
                to_visit.extend(tile.neighbors(pos));
            }
        }
    }

    Ok(map
        .values()
        .filter(|tile| !tile.is_loop && tile.is_outside == [false; 4])
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p2ex1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(4, part2(input).unwrap());
    }

    #[test]
    fn p2ex2() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!(4, part2(input).unwrap());
    }

    #[test]
    fn p2ex3() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(8, part2(input).unwrap());
    }

    #[test]
    fn p2ex4() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(10, part2(input).unwrap());
    }
}
