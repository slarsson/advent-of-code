use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

fn main() {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let coords: Vec<(i32, i32)> = input
        .split('\n')
        .map(|line| {
            let x = line
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            return (x[0], x[1]);
        })
        .collect();

    println!("part1: {}", part1(coords.clone()));
    println!("part2: {}", part2(coords.clone()));
}

fn part1(coords: Vec<(i32, i32)>) -> i32 {
    return shortest_path(coords.clone().into_iter().take(1024).collect()).unwrap();
}

// slow and steady...
fn part2(coords: Vec<(i32, i32)>) -> String {
    for i in 1024..coords.len() + 1 {
        let res = shortest_path(coords.clone().into_iter().take(i).collect());
        if res.is_none() {
            let c = coords[i - 1];
            return format!("{},{}", c.0, c.1);
        }
    }

    return String::from("noop");
}

fn shortest_path(coords: Vec<(i32, i32)>) -> Option<i32> {
    const GRID_SIZE: i32 = 70;

    let mut blocks: HashSet<(i32, i32)> = HashSet::new();

    for c in coords {
        blocks.insert(c);
    }

    let mut queue = BinaryHeap::new();
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    queue.push(Node {
        pos: (0, 0),
        cost: 0,
    });

    loop {
        let head = match queue.pop() {
            Some(x) => x,
            None => return None,
        };

        if seen.contains(&head.pos) {
            continue;
        }

        seen.insert(head.pos);

        if head.pos == (GRID_SIZE, GRID_SIZE) {
            return Some(head.cost);
        }

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (head.pos.0 + dx, head.pos.1 + dy);

            if next.0 < 0 || next.0 > GRID_SIZE || next.1 < 0 || next.1 > GRID_SIZE {
                continue;
            }

            if let Some(_) = blocks.get(&next) {
                continue;
            }

            queue.push(Node {
                pos: next,
                cost: head.cost + 1,
            });
        }
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Node {
    pos: (i32, i32),
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
