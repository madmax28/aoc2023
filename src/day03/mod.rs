use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Number {
    num: i64,
    pos: (i64, i64),
    len: i64,
}

impl Number {
    fn neighbors(&self) -> Vec<(i64, i64)> {
        let mut res = Vec::new();
        res.push((self.pos.0 - 1, self.pos.1));
        res.push((self.pos.0 + self.len, self.pos.1));
        for x in self.pos.0 - 1..=self.pos.0 + self.len {
            res.push((x, self.pos.1 - 1));
            res.push((x, self.pos.1 + 1));
        }
        res
    }

    fn is_part_number(&self, map: &HashMap<(i64, i64), char>) -> bool {
        self.neighbors().into_iter().any(|p| {
            let c = map.get(&p).unwrap_or(&'.');
            c != &'.' && !c.is_ascii_digit()
        })
    }

    fn gear_pos<'a>(
        &'a self,
        map: &'a HashMap<(i64, i64), char>,
    ) -> impl Iterator<Item = (i64, i64)> + 'a {
        self.neighbors()
            .into_iter()
            .filter(|p| map.get(p).unwrap_or(&'.') == &'*')
    }
}

fn parse(input: &str) -> (HashMap<(i64, i64), char>, Vec<Number>) {
    let mut map = HashMap::new();
    let mut numbers = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut x = 0;
        let mut it = line.chars().peekable();
        while let Some(c) = it.next() {
            map.insert((x, y as i64), c);

            if c.is_ascii_digit() {
                let mut num = c.to_string();
                let pos = (x, y as i64);
                let mut len = 1;
                while let Some(true) = it.peek().map(char::is_ascii_digit) {
                    x += 1;
                    let c = it.next().unwrap();
                    map.insert((x, y as i64), c);
                    num.push(c);
                    len += 1;
                }
                let num = num.parse().unwrap();
                numbers.push(Number { num, pos, len });
            }

            x += 1;
        }
    }
    (map, numbers)
}
pub fn part1(input: &str) -> crate::Result<i64> {
    let (map, numbers) = parse(input);
    let sum = numbers
        .into_iter()
        .filter_map(|n| {
            if n.is_part_number(&map) {
                Some(n.num)
            } else {
                None
            }
        })
        .sum();
    Ok(sum)
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let (map, numbers) = parse(input);
    let mut numbers_by_gear: HashMap<(i64, i64), Vec<Number>> = HashMap::new();
    for num in numbers {
        for pos in num.gear_pos(&map) {
            numbers_by_gear.entry(pos).or_default().push(num.clone());
        }
    }
    let sum = numbers_by_gear
        .into_iter()
        .filter_map(|(_, nums)| {
            if nums.len() == 2 {
                Some(nums[0].num * nums[1].num)
            } else {
                None
            }
        })
        .sum();
    Ok(sum)
}
