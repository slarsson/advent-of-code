use std::collections::HashSet;

fn main() {
    let objects = load();
    println!("part1: {}", can_be_removed(&objects).len());
    println!("part1: {}", part2(&objects));
}

fn load() -> HashSet<(i32, i32)> {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    return input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(y, v)| {
                    if v == '@' {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .collect();
}

fn part2(objects: &HashSet<(i32, i32)>) -> usize {
    let mut state = objects.clone();
    let mut sum = 0;

    loop {
        let removed = can_be_removed(&state);
        if removed.len() == 0 {
            break;
        }

        sum += removed.len();

        for r in removed {
            state.remove(&r);
        }
    }

    return sum;
}

fn can_be_removed(objects: &HashSet<(i32, i32)>) -> Vec<(i32, i32)> {
    const NEIGHBORS: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    return objects
        .iter()
        .filter_map(|(x, y)| {
            let count = NEIGHBORS
                .iter()
                .filter(|(dx, dy)| objects.contains(&(x + dx, y + dy)))
                .count();
            if count < 4 { Some((*x, *y)) } else { None }
        })
        .collect();
}
