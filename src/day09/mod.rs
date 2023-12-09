fn extrapolate(nums: &[i64]) -> i64 {
    if nums.iter().all(|n| n == &0) {
        return 0;
    }

    let diffs: Vec<i64> = nums.windows(2).map(|nums| nums[1] - nums[0]).collect();
    nums.last().unwrap() + extrapolate(&diffs)
}

fn parse(input: &str) -> crate::Result<Vec<Vec<i64>>> {
    let nums = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<i64>, _>>()
        })
        .collect::<Result<_, _>>()?;
    Ok(nums)
}

pub fn part1(input: &str) -> crate::Result<i64> {
    Ok(parse(input)?
        .into_iter()
        .map(|nums| extrapolate(&nums))
        .sum())
}

pub fn part2(input: &str) -> crate::Result<i64> {
    Ok(parse(input)?
        .into_iter()
        .map(|mut nums| {
            nums.reverse();
            extrapolate(&nums)
        })
        .sum())
}
