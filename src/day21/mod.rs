use std::{
    collections::{HashMap, HashSet},
    iter,
};

fn neighbors(p: (i64, i64)) -> impl Iterator<Item = (i64, i64)> {
    [
        (p.0 - 1, p.1),
        (p.0 + 1, p.1),
        (p.0, p.1 - 1),
        (p.0, p.1 + 1),
    ]
    .into_iter()
}

fn plots(map: &HashMap<(i64, i64), char>, start: (i64, i64), steps: i64) -> i64 {
    let mut frontier = HashSet::new();
    frontier.insert(start);
    for _ in 0..steps {
        let mut tmp = HashSet::new();
        for p in frontier {
            for np in neighbors(p) {
                if matches!(map.get(&np), Some('.') | Some('S')) {
                    tmp.insert(np);
                }
            }
        }
        frontier = tmp;
    }
    frontier.len() as i64
}

fn parse(input: &str) -> (HashMap<(i64, i64), char>, i64) {
    let mut sz = 0;
    let map: HashMap<(i64, i64), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| iter::repeat(y).zip(l.chars().enumerate()))
        .map(|(y, (x, c))| {
            sz = sz.max(1 + x as i64);
            ((x as i64, y as i64), c)
        })
        .collect();
    (map, sz)
}

pub fn part1(input: &str) -> crate::Result<i64> {
    let (map, sz) = parse(input);
    Ok(plots(&map, (sz / 2, sz / 2), 64))
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let (map, sz) = parse(input);

    const STEPS: i64 = 26501365;
    let skip = STEPS / sz;
    let steps_remaining = STEPS % sz;

    let odd = 1 + (0..skip).map(|x| x * 4).step_by(2).sum::<i64>();
    let even = (0..skip).map(|x| x * 4).skip(1).step_by(2).sum::<i64>();

    let mut num_plots = 0;
    num_plots += plots(&map, (sz / 2, sz / 2), sz) * odd;
    num_plots += plots(&map, (sz / 2, sz / 2), sz - 1) * even;

    num_plots += plots(&map, (0, sz / 2), sz - 1);
    num_plots += plots(&map, (sz - 1, sz / 2), sz - 1);
    num_plots += plots(&map, (sz / 2, 0), sz - 1);
    num_plots += plots(&map, (sz / 2, sz - 1), sz - 1);

    num_plots += plots(&map, (0, 0), steps_remaining - 1) * skip;
    num_plots += plots(&map, (0, sz - 1), steps_remaining - 1) * skip;
    num_plots += plots(&map, (sz - 1, 0), steps_remaining - 1) * skip;
    num_plots += plots(&map, (sz - 1, sz - 1), steps_remaining - 1) * skip;

    num_plots += plots(&map, (0, 0), 3 * (sz / 2)) * (skip - 1);
    num_plots += plots(&map, (0, sz - 1), 3 * (sz / 2)) * (skip - 1);
    num_plots += plots(&map, (sz - 1, 0), 3 * (sz / 2)) * (skip - 1);
    num_plots += plots(&map, (sz - 1, sz - 1), 3 * (sz / 2)) * (skip - 1);
    Ok(num_plots)
}
