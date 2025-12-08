use std::collections::{HashMap, HashSet};

fn main() {
    println!("part1: {:?}", part1());
    println!("part2: {:?}", part2());
}

fn load() -> Vec<(i64, i64, i64)> {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    return input
        .lines()
        .map(|line| {
            let coord: Vec<i64> = line.split(',').map(|v| v.parse::<i64>().unwrap()).collect();
            assert!(coord.len() == 3);
            return (coord[0], coord[1], coord[2]);
        })
        .collect();
}

fn distance(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> f64 {
    let sum = ((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)) as f64;
    return sum.sqrt();
}

fn closest_pairs(coords: &Vec<(i64, i64, i64)>) -> Vec<((i64, i64, i64), (i64, i64, i64), f64)> {
    let mut pairs: Vec<((i64, i64, i64), (i64, i64, i64), f64)> = Vec::new();
    let mut seen: HashSet<((i64, i64, i64), (i64, i64, i64))> = HashSet::new();

    for i in 0..coords.len() {
        for j in 0..coords.len() {
            if i == j {
                continue;
            }

            let a = coords[i];
            let b = coords[j];

            if seen.get(&(a, b)).is_some() || seen.get(&(b, a)).is_some() {
                continue;
            }

            pairs.push((a, b, distance(&a, &b)));

            seen.insert((a, b));
            seen.insert((b, a));
        }
    }

    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    return pairs;
}

fn part1() -> i64 {
    let coords = load();
    let pairs = closest_pairs(&coords);

    let mut g = Group::new();

    for p in &pairs[..1000] {
        g.add(p.0, p.1);
    }

    let mut sizes = g.get_group_sizes();
    sizes.sort_by_key(|x| std::cmp::Reverse(*x));
    return sizes[..3].iter().fold(1i64, |acc, &v| acc * v as i64);
}

fn part2() -> i64 {
    let coords = load();
    let pairs = closest_pairs(&coords);

    let mut g = Group::new();

    for p in pairs {
        g.add(p.0, p.1);

        if g.get_group_size() == 1 && g.get_unique_items_count() == coords.len() {
            return p.0.0 * p.1.0;
        }
    }

    panic!("nope");
}

struct Group {
    groups: HashMap<i32, Vec<(i64, i64, i64)>>,
    lookup: HashMap<(i64, i64, i64), i32>,
    offset: i32,
}

impl Group {
    fn new() -> Self {
        Group {
            groups: HashMap::new(),
            lookup: HashMap::new(),
            offset: 0,
        }
    }

    fn get_group_size(&self) -> usize {
        self.groups.len()
    }

    fn get_unique_items_count(&self) -> usize {
        self.lookup.len()
    }

    fn get_group_sizes(&self) -> Vec<usize> {
        self.groups.iter().map(|(_, v)| v.len()).collect()
    }

    fn add(&mut self, a: (i64, i64, i64), b: (i64, i64, i64)) {
        let a_id = self.lookup.get(&a).cloned();
        let b_id = self.lookup.get(&b).cloned();

        match (a_id, b_id) {
            (None, None) => {
                self.groups.insert(self.offset, vec![a, b]);
                self.lookup.insert(a, self.offset);
                self.lookup.insert(b, self.offset);
                self.offset += 1;
            }
            (Some(a_value), Some(b_value)) => {
                if a_value != b_value {
                    let mut a_list = self.groups.get(&a_value).unwrap().clone();
                    let b_list = self.groups.get(&b_value).unwrap();

                    for v in b_list {
                        a_list.push(*v);
                        self.lookup.insert(*v, a_value);
                    }

                    self.groups.insert(a_value, a_list);
                    self.groups.remove(&b_value);
                }
            }
            (Some(a_value), None) => {
                let mut a_list: Vec<(i64, i64, i64)> = self.groups.get(&a_value).unwrap().clone();
                a_list.push(b);
                self.groups.insert(a_value, a_list);
                self.lookup.insert(b, a_value);
            }
            (None, Some(b_value)) => {
                let mut b_list: Vec<(i64, i64, i64)> = self.groups.get(&b_value).unwrap().clone();
                b_list.push(a);
                self.groups.insert(b_value, b_list);
                self.lookup.insert(a, b_value);
            }
        }
    }
}
