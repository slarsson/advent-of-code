use std::collections::BinaryHeap;

fn main() {
    println!("part1: {}", part1());
}

fn part1() -> u64 {
    let inputs = load();

    let mut sum = 0;
    for (target, masks, _) in inputs {
        sum += solve(target, masks);
    }

    return sum;
}

fn solve(target: u16, masks: Vec<u16>) -> u64 {
    let mut heap = BinaryHeap::new();
    heap.push(Step { steps: 0, value: 0 });

    while let Some(v) = heap.pop() {
        if v.value == target {
            return v.steps;
        }

        for m in &masks {
            heap.push(Step {
                steps: v.steps + 1,
                value: flip(v.value, *m),
            })
        }
    }

    panic!("nope :(");
}

fn flip(bits: u16, mask: u16) -> u16 {
    return bits ^ mask;
}

#[derive(Debug, Eq, PartialEq)]
struct Step {
    steps: u64,
    value: u16,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn load() -> Vec<(u16, Vec<u16>, Vec<u32>)> {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let mut out = Vec::new();

    for line in input.lines() {
        let groups: Vec<&str> = line.split(' ').collect();

        let target = groups
            .first()
            .unwrap()
            .chars()
            .filter(|c| c.eq(&'.') || c.eq(&'#'))
            .enumerate()
            .fold(0, |acc, v| {
                if v.1 == '#' {
                    return acc | (1 << v.0);
                }
                return acc;
            });

        let masks: Vec<u16> = groups[1..groups.len() - 1]
            .iter()
            .map(|v| {
                v.chars()
                    .filter_map(|c| c.to_digit(10))
                    .fold(0, |acc, d| acc | (1 << d))
            })
            .collect();

        let digits = groups
            .last()
            .unwrap()
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|v| v.parse::<u32>().unwrap())
            .collect();

        out.push((target, masks, digits));
    }

    return out;
}
