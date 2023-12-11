use std::cmp::{max, min};
use std::collections::HashSet;
use std::iter;

fn solve(input: &str, expand: usize) -> crate::Result<usize> {
    let stars: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| iter::repeat(y).zip(l.chars().enumerate()))
        .filter(|(_, (_, c))| c == &'#')
        .map(|(y, (x, _))| (x, y))
        .collect();
    let rows: HashSet<usize> = stars.iter().map(|(_, y)| *y).collect();
    let cols: HashSet<usize> = stars.iter().map(|(x, _)| *x).collect();

    let sum = stars
        .iter()
        .enumerate()
        .flat_map(|(idx, star)| iter::repeat(star).zip(&stars[idx + 1..]))
        .map(|((x1, y1), (x2, y2))| {
            let mut sum: usize = (1 + min(*x1, *x2)..=max(*x1, *x2))
                .map(|x| if cols.contains(&x) { 1 } else { expand })
                .sum::<usize>();
            sum += (1 + min(*y1, *y2)..=max(*y1, *y2))
                .map(|y| if rows.contains(&y) { 1 } else { expand })
                .sum::<usize>();
            sum
        })
        .sum();
    Ok(sum)
}

pub fn part1(input: &str) -> crate::Result<usize> {
    solve(input, 2)
}

pub fn part2(input: &str) -> crate::Result<usize> {
    solve(input, 1000000)
}
