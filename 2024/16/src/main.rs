use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn main() {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut start: (i32, i32) = (0, 0);
    let start_dir = (1, 0);
    let mut end: (i32, i32) = (0, 0);

    for (y, line) in input.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    map.insert((x as i32, y as i32));
                }
                'S' => {
                    start = (x as i32, y as i32);
                }
                'E' => {
                    end = (x as i32, y as i32);
                }
                _ => {}
            }
        }
    }

    let res = solve(start, start_dir, end, &map);

    println!("part1: {}", res.0);
    println!("part2: {}", res.1);
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Node {
    pos: (i32, i32),
    dir: (i32, i32),
    cost: i32,
    prev: Option<(i32, i32, i32, i32)>,
}

impl Node {
    fn key(&self) -> (i32, i32, i32, i32) {
        return (self.pos.0, self.pos.1, self.dir.0, self.dir.1);
    }
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

fn solve(
    start: (i32, i32),
    start_dir: (i32, i32),
    end: (i32, i32),
    map: &HashSet<(i32, i32)>,
) -> (i32, i32) {
    let mut edges: HashMap<(i32, i32, i32, i32), Vec<(i32, i32, i32, i32, i32)>> = HashMap::new();
    let mut vertices: HashMap<(i32, i32, i32, i32), Node> = HashMap::new();
    let mut queue = BinaryHeap::new();

    queue.push(Node {
        pos: start,
        dir: start_dir,
        cost: 0,
        prev: None,
    });

    loop {
        let head = queue.pop().unwrap();

        if let Some(x) = vertices.get(&head.key()) {
            if head.cost < x.cost {
                vertices.insert(head.key().clone(), head.clone());
            } else {
                continue;
            }
        } else {
            vertices.insert(head.key().clone(), head.clone());
        }

        if head.pos == end {
            return (
                head.cost,
                count_all_path_tiles(head.clone(), &vertices, &edges),
            );
        }

        let next_forward = (head.pos.0 + head.dir.0, head.pos.1 + head.dir.1);
        if !map.contains(&next_forward) {
            edges
                .entry((head.pos.0, head.pos.1, head.dir.0, head.dir.1))
                .or_default()
                .push((
                    next_forward.0,
                    next_forward.1,
                    head.dir.0,
                    head.dir.1,
                    head.cost + 1,
                ));

            queue.push(Node {
                pos: next_forward,
                dir: head.dir,
                cost: head.cost + 1,
                prev: Some((head.pos.0, head.pos.1, head.dir.0, head.dir.1)),
            });
        }

        let l_rot = rotate_90((head.dir.0, head.dir.1));
        let r_rot = (l_rot.0 * -1, l_rot.1 * -1);

        let next_l_rot = (head.pos.0 + l_rot.0, head.pos.1 + l_rot.1);
        if !map.contains(&next_l_rot) {
            edges
                .entry((head.pos.0, head.pos.1, head.dir.0, head.dir.1))
                .or_default()
                .push((
                    next_l_rot.0,
                    next_l_rot.1,
                    l_rot.0,
                    l_rot.1,
                    head.cost + 1001,
                ));

            queue.push(Node {
                pos: next_l_rot,
                dir: l_rot,
                cost: head.cost + 1001,
                prev: Some((head.pos.0, head.pos.1, head.dir.0, head.dir.1)),
            });
        }

        let next_r_rot = (head.pos.0 + r_rot.0, head.pos.1 + r_rot.1);
        if !map.contains(&next_r_rot) {
            edges
                .entry((head.pos.0, head.pos.1, head.dir.0, head.dir.1))
                .or_default()
                .push((
                    next_r_rot.0,
                    next_r_rot.1,
                    r_rot.0,
                    r_rot.1,
                    head.cost + 1001,
                ));

            queue.push(Node {
                pos: next_r_rot,
                dir: r_rot,
                cost: head.cost + 1001,
                prev: Some((head.pos.0, head.pos.1, head.dir.0, head.dir.1)),
            });
        }
    }
}

fn rotate_90(dir: (i32, i32)) -> (i32, i32) {
    // (x, y) -> (-y, x)
    return (-dir.1, dir.0);
}

fn count_all_path_tiles(
    start_node: Node,
    vertices: &HashMap<(i32, i32, i32, i32), Node>,
    edges: &HashMap<(i32, i32, i32, i32), Vec<(i32, i32, i32, i32, i32)>>,
) -> i32 {
    let mut tiles: HashSet<(i32, i32)> = HashSet::new();
    let mut seen: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let mut cur: Vec<(i32, i32, i32, i32)> = Vec::new();

    cur.push(start_node.key());

    loop {
        if cur.len() == 0 {
            break;
        }

        let mut cur_next: Vec<(i32, i32, i32, i32)> = Vec::new();

        for c in &cur {
            if seen.contains(c) {
                continue;
            }

            seen.insert(*c);
            tiles.insert((c.0, c.1));

            let node = vertices.get(&c);

            if node.is_none() {
                continue;
            }

            if let Some(prev) = node.unwrap().prev {
                cur_next.push(prev);

                for (from, tos) in edges {
                    if *from == prev {
                        continue;
                    }

                    let to_node = tos
                        .iter()
                        .find(|x| x.0 == c.0 && x.1 == c.1 && x.2 == c.2 && x.3 == c.3);

                    match to_node {
                        Some(e) => {
                            if e.0 == prev.0 && e.1 == prev.1 && e.2 == prev.2 && e.3 == prev.3 {
                                continue;
                            }

                            // only take edge with same cost
                            if e.4 == node.unwrap().cost {
                                cur_next.push(*from);
                            }
                        }
                        None => {}
                    }
                }
            }
        }

        cur = cur_next;
    }

    return tiles.len() as i32;
}
