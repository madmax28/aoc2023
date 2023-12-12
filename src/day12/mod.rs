use std::collections::HashMap;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn parse(input: &str) -> crate::Result<Vec<(&str, Vec<usize>)>> {
    let mut res = Vec::new();
    for line in input.lines() {
        if let Some((springs, instr)) = line.split_once(' ') {
            let instr = instr
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<usize>, _>>()?;
            res.push((springs, instr));
        } else {
            return Err(crate::Error::boxed(Error::InvalidInput));
        }
    }
    Ok(res)
}

fn num_options<'a>(
    springs: &'a str,
    instr: &'a [usize],
    solved: &mut HashMap<(&'a str, &'a [usize]), usize>,
) -> usize {
    if let Some(num_opts) = solved.get(&(springs, instr)) {
        return *num_opts;
    }

    if instr.is_empty() {
        if springs.chars().all(|c| c != '#') {
            return 1;
        } else {
            return 0;
        };
    }

    let len = instr[0];
    if springs.len() < len {
        return 0;
    }

    if springs.len() == len {
        if springs.chars().all(|c| c != '.') && instr.len() == 1 {
            return 1;
        } else {
            return 0;
        };
    }

    let mut num_opts = 0;
    if springs[..len].chars().all(|c| c != '.') && springs.chars().nth(len).unwrap() != '#' {
        num_opts += num_options(&springs[len + 1..], &instr[1..], solved);
    }
    if !springs.starts_with('#') {
        num_opts += num_options(&springs[1..], instr, solved);
    }
    solved.insert((springs, instr), num_opts);
    num_opts
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let sum = parse(input)?
        .into_iter()
        .map(|(springs, instr)| num_options(springs, &instr, &mut HashMap::new()))
        .sum();
    Ok(sum)
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let sum = parse(input)?
        .into_iter()
        .map(|(row, instr)| {
            let row = [row; 5].join("?");
            let instr: Vec<usize> = instr
                .iter()
                .cloned()
                .cycle()
                .take(5 * instr.len())
                .collect();
            (row, instr)
        })
        .map(|(springs, instr)| num_options(&springs, &instr, &mut HashMap::new()))
        .sum();
    Ok(sum)
}
