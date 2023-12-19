use std::collections::HashMap;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug)]
struct Part(u64, u64, u64, u64);

impl Part {
    fn sum(self) -> u64 {
        self.0 + self.1 + self.2 + self.3
    }
}

#[derive(Debug, Clone)]
struct PartRange {
    xrange: (u64, u64),
    mrange: (u64, u64),
    arange: (u64, u64),
    srange: (u64, u64),
}

impl PartRange {
    fn count(&self) -> u64 {
        (1 + self.xrange.1 - self.xrange.0)
            * (1 + self.mrange.1 - self.mrange.0)
            * (1 + self.arange.1 - self.arange.0)
            * (1 + self.srange.1 - self.srange.0)
    }
}

#[derive(Debug)]
enum Rule<'a> {
    Conditional((char, char, u64, &'a str)),
    Unconditional(&'a str),
}

impl Rule<'_> {
    fn apply(&self, part: &Part) -> Option<&str> {
        match self {
            Self::Conditional((prop, op, val, res)) => {
                let lhs = match prop {
                    'x' => part.0,
                    'm' => part.1,
                    'a' => part.2,
                    's' => part.3,
                    _ => unreachable!(),
                };

                if match op {
                    '<' => lhs < *val,
                    '>' => lhs > *val,
                    _ => unreachable!(),
                } {
                    Some(res)
                } else {
                    None
                }
            }
            Self::Unconditional(res) => Some(res),
        }
    }

    fn apply_range(&self, range: PartRange) -> (&str, Option<PartRange>, Option<PartRange>) {
        match self {
            Self::Conditional((prop, op, val, res)) => {
                let (mut matched, mut unmatched) = (range.clone(), range.clone());
                let (range, matched_range, unmatched_range) = match prop {
                    'x' => (range.xrange, &mut matched.xrange, &mut unmatched.xrange),
                    'm' => (range.mrange, &mut matched.mrange, &mut unmatched.mrange),
                    'a' => (range.arange, &mut matched.arange, &mut unmatched.arange),
                    's' => (range.srange, &mut matched.srange, &mut unmatched.srange),
                    _ => unreachable!(),
                };

                match op {
                    '<' => {
                        if range.0 < *val {
                            if range.1 < *val {
                                // full range matches
                                (res, Some(matched), None)
                            } else {
                                // part matches
                                matched_range.1 = *val - 1;
                                unmatched_range.0 = *val;
                                (res, Some(matched), Some(unmatched))
                            }
                        } else {
                            // nothing matches
                            (res, None, Some(unmatched))
                        }
                    }
                    '>' => {
                        if range.1 > *val {
                            if range.0 > *val {
                                // full range matches
                                (res, Some(matched), None)
                            } else {
                                // part matches
                                matched_range.0 = *val + 1;
                                unmatched_range.1 = *val;
                                (res, Some(matched), Some(unmatched))
                            }
                        } else {
                            // nothing matches
                            (res, None, Some(unmatched))
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Self::Unconditional(res) => (res, Some(range.clone()), None),
        }
    }
}

#[derive(Debug)]
struct Flow<'a>(Vec<Rule<'a>>);

impl Flow<'_> {
    fn apply(&self, part: &Part) -> &str {
        for rule in &self.0 {
            if let Some(res) = rule.apply(part) {
                return res;
            }
        }
        unreachable!()
    }

    fn apply_range(&self, range: PartRange) -> Vec<(&str, PartRange)> {
        let mut matched = Vec::new();
        let mut unmatched = vec![range.clone()];
        for rule in &self.0 {
            let mut tmp = Vec::new();
            for range in unmatched {
                let (res, m, u) = rule.apply_range(range);
                if let Some(m) = m {
                    matched.push((res, m));
                }
                if let Some(u) = u {
                    tmp.push(u);
                }
            }
            unmatched = tmp;
        }
        assert!(unmatched.is_empty());
        matched
    }
}

type Flows<'a> = HashMap<&'a str, Flow<'a>>;

fn parse(input: &str) -> crate::Result<(Flows, Vec<Part>)> {
    let (flow_str, part_str) = input
        .split_once("\n\n")
        .ok_or(crate::Error::boxed(Error::InvalidInput))?;

    let mut flows = Flows::new();
    for flow in flow_str.lines() {
        let curly = flow
            .find('{')
            .ok_or(crate::Error::boxed(Error::InvalidInput))?;

        let id = &flow[..curly];
        let mut rules = Vec::new();
        for rule in flow[curly + 1..flow.len() - 1].split(',') {
            if let Some((cond, result)) = rule.split_once(':') {
                let prop = cond
                    .chars()
                    .next()
                    .ok_or(crate::Error::boxed(Error::InvalidInput))?;
                let op = cond
                    .chars()
                    .nth(1)
                    .ok_or(crate::Error::boxed(Error::InvalidInput))?;
                if op != '<' && op != '>' {
                    return Err(crate::Error::boxed(Error::InvalidInput));
                }
                let val: u64 = cond[2..].parse()?;
                rules.push(Rule::Conditional((prop, op, val, result)));
            } else {
                rules.push(Rule::Unconditional(rule));
            }
        }
        flows.insert(id, Flow(rules));
    }

    let mut parts = Vec::new();
    for part in part_str.lines() {
        let values = part
            .split(',')
            .map(|s| s.trim_matches(|c: char| !c.is_ascii_digit()).parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()?;
        if values.len() != 4 {
            return Err(crate::Error::boxed(Error::InvalidInput));
        }
        parts.push(Part(values[0], values[1], values[2], values[3]));
    }

    Ok((flows, parts))
}

pub fn part1(input: &str) -> crate::Result<u64> {
    let (flows, parts) = parse(input)?;
    let mut sum = 0;
    for part in parts {
        let mut id = "in";
        while id != "A" && id != "R" {
            let flow = flows
                .get(id)
                .ok_or(crate::Error::boxed(Error::InvalidInput))?;
            id = flow.apply(&part);
        }

        if id == "A" {
            sum += part.sum();
        }
    }
    Ok(sum)
}

pub fn part2(input: &str) -> crate::Result<u64> {
    let (flows, _) = parse(input)?;
    let mut sum = 0;
    let mut ranges = vec![(
        "in",
        PartRange {
            xrange: (1, 4000),
            mrange: (1, 4000),
            arange: (1, 4000),
            srange: (1, 4000),
        },
    )];
    while let Some((id, range)) = ranges.pop() {
        let flow = flows
            .get(id)
            .ok_or(crate::Error::boxed(Error::InvalidInput))?;
        for (id, range) in flow.apply_range(range) {
            match id {
                "A" => sum += range.count(),
                "R" => (),
                id => ranges.push((id, range)),
            }
        }
    }
    Ok(sum)
}
