use std::collections::HashMap;

#[derive(Debug)]
struct Number {
    num: i32,
    pos: (i32, i32),
    len: i32,
}

impl Number {
    fn neighbors(&self) -> Vec<(i32, i32)> {
        let mut res = Vec::new();
        res.push((self.pos.0 - 1, self.pos.1));
        res.push((self.pos.0 + self.len, self.pos.1));
        for x in self.pos.0 - 1..=self.pos.0 + self.len {
            res.push((x, self.pos.1 - 1));
            res.push((x, self.pos.1 + 1));
        }
        res
    }

    fn is_part_number(&self, map: &HashMap<(i32, i32), char>) -> bool {
        self.neighbors()
            .into_iter()
            .any(|p| map.get(&p).unwrap_or(&'.') != &'.')
    }

    fn gear_pos(&self, map: &HashMap<(i32, i32), char>) -> Option<(i32, i32)> {
        self.neighbors()
            .into_iter()
            .find(|p| map.get(p).unwrap_or(&'.') == &'*')
    }
}

fn parse(input: &str) -> (HashMap<(i32, i32), char>, Vec<Number>) {
    let mut map = HashMap::new();
    let mut numbers = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut x = 0;
        let mut it = line.chars().peekable();
        while let Some(c) = it.next() {
            map.insert((x, y as i32), c);

            if c.is_ascii_digit() {
                let mut num = c.to_string();
                let pos = (x, y as i32);
                let mut len = 1;
                while let Some(true) = it.peek().map(char::is_ascii_digit) {
                    x += 1;
                    let c = it.next().unwrap();
                    map.insert((x, y as i32), c);
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
pub fn part1(input: &str) -> crate::Result<i32> {
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

pub fn part2(input: &str) -> crate::Result<i32> {
    let (map, numbers) = parse(input);
    let mut numbers_by_gear: HashMap<(i32, i32), Vec<Number>> = HashMap::new();
    for num in numbers {
        if let Some(pos) = num.gear_pos(&map) {
            numbers_by_gear.entry(pos).or_default().push(num);
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
