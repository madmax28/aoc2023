#[derive(Debug)]
enum Error {
    InvalidInput,
}

pub fn part1(input: &str) -> crate::Result<u32> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .ok_or(crate::Error::boxed(Error::InvalidInput))?
        .split_whitespace()
        .skip(1);
    let dists = lines
        .next()
        .ok_or(crate::Error::boxed(Error::InvalidInput))?
        .split_whitespace()
        .skip(1);

    let mut res = 1;
    for (time, dist) in times.zip(dists) {
        let time: u32 = time.parse()?;
        let dist: u32 = dist.parse()?;

        let mut cnt = 0;
        for cand in 1..time {
            if cand * (time - cand) > dist {
                cnt += 1;
            }
        }
        res *= cnt;
    }
    Ok(res)
}

pub fn part2(input: &str) -> crate::Result<u32> {
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .ok_or(crate::Error::boxed(Error::InvalidInput))?
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()?;
    let dist: u64 = lines
        .next()
        .ok_or(crate::Error::boxed(Error::InvalidInput))?
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()?;

    let mut cnt = 0;
    for cand in 1..time {
        if cand * (time - cand) > dist {
            cnt += 1;
        }
    }
    Ok(cnt)
}
