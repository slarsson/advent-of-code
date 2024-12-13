use std::collections::{HashMap, HashSet};

fn main() {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), ch);
        }
    }

    let res = solve(&grid);

    println!("part1: {:?}", res.0);
    println!("part2: {:?}", res.1);
}

fn solve(grid: &HashMap<(i32, i32), char>) -> (i32, i32) {
    let mut all_sides = 0;
    let mut unique_sides = 0;
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    for pos in grid {
        if seen.contains(&pos.0) {
            continue;
        }

        let mut group: HashSet<(i32, i32)> = HashSet::new();
        group.insert(*pos.0);

        build_group(*pos.0, *pos.1, &grid, &mut seen, &mut group);

        all_sides += group.len() as i32 * find_all_sides(&group);
        unique_sides += group.len() as i32 * find_unique_sides(&mut group);
    }

    return (all_sides, unique_sides);
}

fn build_group(
    pos: (i32, i32),
    value: char,
    grid: &HashMap<(i32, i32), char>,
    seen: &mut HashSet<(i32, i32)>,
    group: &mut HashSet<(i32, i32)>,
) {
    if seen.contains(&pos) {
        return;
    }

    seen.insert(pos);

    for delta in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let cur = (pos.0 + delta.0, pos.1 + delta.1);

        if let Some(&c) = grid.get(&cur) {
            if c != value {
                continue;
            }

            group.insert(cur);

            build_group(cur, value, grid, seen, group);
        }
    }
}

fn find_all_sides(group: &HashSet<(i32, i32)>) -> i32 {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut sides = 0;

    for pos in group.iter() {
        if seen.contains(&pos) {
            continue;
        }

        seen.insert(*pos);

        for delta in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let neighbor = (pos.0 + delta.0, pos.1 + delta.1);

            if group.get(&neighbor).is_none() {
                sides += 1;
            }
        }
    }

    return sides;
}

fn find_unique_sides(group: &mut HashSet<(i32, i32)>) -> i32 {
    let mut border: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut sides = 0;

    for pos in group.iter() {
        // third number indicates border orientation
        for delta in [(0, 1, 1), (0, -1, 2), (1, 0, 3), (-1, 0, 4)] {
            let neighbor = (pos.0 + delta.0, pos.1 + delta.1);

            if group.get(&neighbor).is_none() {
                sides += 1;
                border.insert((neighbor.0, neighbor.1, delta.2));
            }
        }
    }

    return sides - count_extra_sides(&border);
}

fn count_extra_sides(border: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut seen: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut count = 0;

    for pos in border {
        let length = get_path_length(*pos, border, &mut seen);
        if length > 1 {
            count += length - 1;
        }
    }

    return count;
}

fn get_path_length(
    pos: (i32, i32, i32),
    border: &HashSet<(i32, i32, i32)>,
    seen: &mut HashSet<(i32, i32, i32)>,
) -> i32 {
    if seen.contains(&pos) {
        return 0;
    }

    seen.insert(pos);

    let mut length = 1;

    for delta in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let neighbor = (pos.0 + delta.0, pos.1 + delta.1, pos.2);

        if border.contains(&neighbor) {
            length += get_path_length(neighbor, border, seen);
        }
    }

    return length;
}
