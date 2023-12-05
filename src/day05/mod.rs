use std::{cmp::min, str::FromStr};

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug, Clone, PartialEq)]
struct Range {
    start: i64,
    length: i64,
}

impl Range {
    fn new(start: i64, length: i64) -> Self {
        Range { start, length }
    }

    fn intersect(&self, range: &Range) -> (Option<Range>, Vec<Range>) {
        if self.start < range.start + range.length && self.start + self.length > range.start {
            let mut start = range.start;

            // unmatched range at start
            let mut unmatched = Vec::new();
            if start < self.start {
                unmatched.push(Range::new(start, self.start - range.start));
                start += self.start - range.start;
            }

            // matched range
            let end = min(self.start + self.length, range.start + range.length);
            let length = end - start;
            let matched = Range::new(start, length);
            start += length;

            // unmatched range at end
            if range.start + range.length > self.start + self.length {
                unmatched.push(Range::new(
                    start,
                    range.start + range.length - self.start - self.length,
                ));
            }

            (Some(matched), unmatched)
        } else {
            (None, vec![range.clone()])
        }
    }
}

#[derive(Debug, Clone)]
struct Mapping {
    dst_start: i64,
    src_start: i64,
    length: i64,
}

impl Mapping {
    fn try_map(&self, src: i64) -> Option<i64> {
        if (self.src_start..self.src_start + self.length).contains(&src) {
            Some(src + self.dst_start - self.src_start)
        } else {
            None
        }
    }

    fn map_range(&self, range: &Range) -> (Option<Range>, Vec<Range>) {
        match Range::new(self.src_start, self.length).intersect(range) {
            (Some(mut mapped), unmapped) => {
                mapped.start = self.try_map(mapped.start).unwrap();
                (Some(mapped), unmapped)
            }
            res => res,
        }
    }
}

impl FromStr for Mapping {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        if let &[Ok(dst_start), Ok(src_start), Ok(length)] =
            &s.split_whitespace().map(str::parse).collect::<Vec<_>>()[..3]
        {
            Ok(Mapping {
                dst_start,
                src_start,
                length,
            })
        } else {
            Err(crate::Error::boxed(Error::InvalidInput))
        }
    }
}

#[derive(Debug, Clone)]
struct Table {
    mappings: Vec<Mapping>,
}

impl Table {
    fn map_range(&self, range: &Range) -> Vec<Range> {
        let mut unmapped = vec![range.clone()];
        let mut mapped = Vec::new();
        for mapping in &self.mappings {
            let mut tmp = Vec::new();
            while let Some(range) = unmapped.pop() {
                match mapping.map_range(&range) {
                    (Some(m), u) => {
                        mapped.push(m);
                        unmapped.extend(u);
                    }
                    (None, u) => tmp.extend(u),
                }
            }
            unmapped = tmp;
        }
        mapped.extend(unmapped);
        mapped
    }
}

impl FromStr for Table {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        let mappings = s
            .lines()
            .skip(1)
            .map(Mapping::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Table { mappings })
    }
}

pub fn part1(input: &str) -> crate::Result<i64> {
    let mut parts = input.split("\n\n");
    let seeds = parts
        .next()
        .ok_or(crate::Error::boxed(Error::InvalidInput))?
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let tables = parts.map(Table::from_str).collect::<Result<Vec<_>, _>>()?;

    let min = seeds
        .into_iter()
        .map(|mut src| {
            for table in &tables {
                src = table
                    .mappings
                    .iter()
                    .find_map(|t| t.try_map(src))
                    .unwrap_or(src);
            }
            src
        })
        .min()
        .ok_or(crate::Error::boxed(Error::InvalidInput))?;
    Ok(min)
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let mut parts = input.split("\n\n");
    let seeds = parts
        .next()
        .ok_or(crate::Error::boxed(Error::InvalidInput))?
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?;
    let mut seeds: Vec<Range> = seeds
        .chunks_exact(2)
        .map(|chunk| Range::new(chunk[0], chunk[1]))
        .collect();
    let tables = parts.map(Table::from_str).collect::<Result<Vec<_>, _>>()?;

    for table in &tables {
        let mut tmp = Vec::new();
        for seed in &seeds {
            tmp.extend(table.map_range(seed));
        }
        seeds = tmp;
    }
    Ok(seeds
        .into_iter()
        .map(|s| s.start)
        .min()
        .ok_or(crate::Error::boxed(Error::InvalidInput))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_intersect() {
        let range = Range::new(10, 10);

        let (matched, unmatched) = range.intersect(&Range::new(2, 5));
        assert!(matched.is_none());
        assert_eq!(1, unmatched.len());
        assert_eq!(Range::new(2, 5), unmatched[0]);

        let (matched, unmatched) = range.intersect(&Range::new(22, 5));
        assert!(matched.is_none());
        assert_eq!(1, unmatched.len());
        assert_eq!(Range::new(22, 5), unmatched[0]);

        let (matched, unmatched) = range.intersect(&Range::new(12, 5));
        assert_eq!(Range::new(12, 5), matched.unwrap());
        assert!(unmatched.is_empty());

        let (matched, unmatched) = range.intersect(&Range::new(8, 5));
        assert_eq!(Range::new(10, 3), matched.unwrap());
        assert_eq!(1, unmatched.len());
        assert_eq!(Range::new(8, 2), unmatched[0]);

        let (matched, unmatched) = range.intersect(&Range::new(18, 5));
        assert_eq!(Range::new(18, 2), matched.unwrap());
        assert_eq!(1, unmatched.len());
        assert_eq!(Range::new(20, 3), unmatched[0]);

        let (matched, unmatched) = range.intersect(&Range::new(8, 15));
        assert_eq!(Range::new(10, 10), matched.unwrap());
        assert_eq!(2, unmatched.len());
        assert_eq!(Range::new(8, 2), unmatched[0]);
        assert_eq!(Range::new(20, 3), unmatched[1]);
    }

    #[test]
    fn ex1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(46, part2(input).unwrap());
    }
}
