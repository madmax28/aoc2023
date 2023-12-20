use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
enum State {
    High,
    Low,
}

impl State {
    fn inv(&mut self) {
        *self = match self {
            State::High => State::Low,
            State::Low => State::High,
        }
    }
}

#[derive(Debug)]
struct Pulse<'a> {
    src: &'a str,
    state: State,
}

impl<'a> Pulse<'a> {
    fn new(src: &'a str, state: State) -> Self {
        Self { src, state }
    }
}

#[derive(Debug, Clone)]
enum Kind<'a> {
    Broadcast,
    FlipFlop(State),
    Conjunction(HashMap<&'a str, State>),
}

#[derive(Debug, Clone)]
struct Module<'a> {
    kind: Kind<'a>,
    outputs: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn new(kind: Kind<'a>, outputs: Vec<&'a str>) -> Self {
        Self { kind, outputs }
    }

    fn pulse(&mut self, src: &'a str, state: State) -> Option<State> {
        match &mut self.kind {
            Kind::Broadcast => Some(state),
            Kind::FlipFlop(flipflop_state) => match state {
                State::Low => {
                    flipflop_state.inv();
                    Some(*flipflop_state)
                }
                _ => None,
            },
            Kind::Conjunction(mem) => {
                mem.insert(src, state);
                if mem.values().all(|state| matches!(state, State::High)) {
                    Some(State::Low)
                } else {
                    Some(State::High)
                }
            }
        }
    }

    fn register_input(&mut self, src: &'a str) {
        if let Kind::Conjunction(mem) = &mut self.kind {
            mem.insert(src, State::Low);
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Module> {
    let mut modules = HashMap::new();
    let mut connectivity = HashMap::new();
    for line in input.lines() {
        let (id, outputs) = line.split_once(" -> ").unwrap();
        let outputs: Vec<&str> = outputs.split(", ").collect();
        match id.chars().next().unwrap() {
            'b' => {
                connectivity.insert(id, outputs.clone());
                modules.insert(id, Module::new(Kind::Broadcast, outputs));
            }
            '%' => {
                connectivity.insert(&id[1..], outputs.clone());
                modules.insert(&id[1..], Module::new(Kind::FlipFlop(State::Low), outputs));
            }
            '&' => {
                connectivity.insert(&id[1..], outputs.clone());
                modules.insert(
                    &id[1..],
                    Module::new(Kind::Conjunction(HashMap::new()), outputs),
                );
            }
            _ => unreachable!(),
        }
    }

    for (src, outputs) in connectivity {
        for tgt in outputs {
            if let Some(module) = modules.get_mut(tgt) {
                module.register_input(src);
            }
        }
    }

    modules
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let mut modules = parse(input);
    let (mut low, mut high) = (0, 0);
    for _ in 0..1000 {
        let mut pulse_queue = VecDeque::new();
        pulse_queue.push_back(Pulse::new("button", State::Low));

        while let Some(pulse) = pulse_queue.pop_front() {
            let outputs = if let Some(module) = modules.get(pulse.src) {
                module.outputs.clone()
            } else {
                vec!["broadcaster"]
            };

            match pulse.state {
                State::Low => low += outputs.len(),
                State::High => high += outputs.len(),
            }

            for target in outputs {
                if let Some(module) = modules.get_mut(target) {
                    if let Some(state) = module.pulse(pulse.src, pulse.state) {
                        pulse_queue.push_back(Pulse::new(target, state));
                    }
                }
            }
        }
    }
    Ok(low * high)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if b == 0 || a == 0 {
        0
    } else {
        a * (b / gcd(a, b))
    }
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let modules = parse(input);

    let last_conj = &modules
        .values()
        .find(|m| m.outputs.contains(&"rx"))
        .unwrap();
    let conj_inputs: Vec<&str> = if let Kind::Conjunction(mem) = &last_conj.kind {
        mem.keys().cloned().collect()
    } else {
        unreachable!()
    };

    let mut press_cnts = Vec::new();
    'outer: for input in conj_inputs {
        let mut modules = modules.clone();
        for press_cnt in 1.. {
            let mut pulse_queue = VecDeque::new();
            pulse_queue.push_back(Pulse::new("button", State::Low));

            while let Some(pulse) = pulse_queue.pop_front() {
                if pulse.src == input && matches!(pulse.state, State::High) {
                    press_cnts.push(press_cnt);
                    continue 'outer;
                }

                let outputs = if let Some(module) = modules.get(pulse.src) {
                    module.outputs.clone()
                } else {
                    vec!["broadcaster"]
                };

                for target in outputs {
                    if let Some(module) = modules.get_mut(target) {
                        if let Some(state) = module.pulse(pulse.src, pulse.state) {
                            pulse_queue.push_back(Pulse::new(target, state));
                        }
                    }
                }
            }
        }
    }
    Ok(press_cnts.into_iter().fold(1, lcm))
}
