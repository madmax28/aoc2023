use std::collections::HashMap;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse(input: &str) -> crate::Result<(&str, Map)> {
    if let Some((directions, network)) = input.split_once("\n\n") {
        let mut map = HashMap::new();
        for line in network.lines() {
            map.insert(&line[..3], (&line[7..10], &line[12..15]));
        }
        Ok((directions, map))
    } else {
        Err(crate::Error::boxed(Error::InvalidInput))
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let (directions, map) = parse(input)?;
    let mut pos = "AAA";
    for (idx, turn) in directions.chars().cycle().enumerate() {
        if pos == "ZZZ" {
            return Ok(idx);
        }

        let dst = map
            .get(pos)
            .ok_or(crate::Error::boxed(Error::InvalidInput))?;
        pos = match turn {
            'L' => dst.0,
            'R' => dst.1,
            _ => return Err(crate::Error::boxed(Error::InvalidInput)),
        }
    }
    unreachable!()
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    if b == 0 || a == 0 {
        0
    } else {
        a * (b / gcd(a, b))
    }
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let (directions, map) = parse(input)?;
    let directions: Vec<char> = directions.chars().collect();

    let mut loops = Vec::new();
    for mut pos in map.keys().filter(|key| key.ends_with('A')).cloned() {
        let mut seen: HashMap<(&str, usize), i64> = HashMap::new();
        for (step_cnt, dir_idx) in (0..directions.len()).cycle().enumerate() {
            let step_cnt = step_cnt as i64;
            if pos.ends_with('Z') {
                if let Some(start_step_cnt) = seen.get(&(pos, dir_idx)) {
                    loops.push(step_cnt - start_step_cnt);
                    break;
                }
                seen.insert((pos, dir_idx), step_cnt);
            }

            let dst = map
                .get(pos)
                .ok_or(crate::Error::boxed(Error::InvalidInput))?;
            pos = match directions[dir_idx] {
                'L' => dst.0,
                'R' => dst.1,
                _ => return Err(crate::Error::boxed(Error::InvalidInput)),
            }
        }
    }
    Ok(loops.into_iter().fold(1, lcm))
}
