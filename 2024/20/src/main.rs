use std::collections::{HashMap, HashSet};

fn main() {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => start = (x as i32, y as i32),
                '#' => continue,
                _ => (),
            }

            map.insert((x as i32, y as i32));
        }
    }

    println!("part1: {}", part1(start, map.clone()));
    println!("part2: {}", part2(start, map.clone()));
}

fn part1(start: (i32, i32), map: HashSet<(i32, i32)>) -> i32 {
    let distances = get_distances(start, &map);
    return valid_cheats(2, &distances);
}

fn part2(start: (i32, i32), map: HashSet<(i32, i32)>) -> i32 {
    let distances = get_distances(start, &map);

    let mut sum = 0;

    for r in 2..21 {
        sum += valid_cheats(r, &distances);
    }

    return sum;
}

fn valid_cheats(r: i32, distances: &HashMap<(i32, i32), i32>) -> i32 {
    let mut sum = 0;

    for (pos, current_distance) in distances {
        for c in manhattan_coords(pos.0, pos.1, r) {
            match distances.get(&c) {
                Some(new_distance) => {
                    if new_distance - (current_distance + r) >= 100 {
                        sum += 1;
                    }
                }
                None => (),
            }
        }
    }

    return sum;
}

fn get_distances(start: (i32, i32), tiles: &HashSet<(i32, i32)>) -> HashMap<(i32, i32), i32> {
    let mut distances: HashMap<(i32, i32), i32> = HashMap::new();
    distances.insert(start, 0);

    let mut distance = 0;
    let mut cur = start;

    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    seen.insert(cur);

    'outer: loop {
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (cur.0 + dx, cur.1 + dy);
            if seen.contains(&next) {
                continue;
            }

            seen.insert(next);

            if tiles.contains(&next) {
                distances.insert(next, distance + 1);
                cur = next;
                distance += 1;
                continue 'outer;
            }
        }

        break;
    }

    return distances;
}

fn manhattan_coords(x_center: i32, y_center: i32, r: i32) -> Vec<(i32, i32)> {
    let mut out: Vec<(i32, i32)> = Vec::new();

    for offset in 0..r {
        let inv_offset = r - offset;
        out.push((x_center + offset, y_center + inv_offset));
        out.push((x_center + inv_offset, y_center - offset));
        out.push((x_center - offset, y_center - inv_offset));
        out.push((x_center - inv_offset, y_center + offset));
    }

    return out;
}
