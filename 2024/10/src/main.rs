use std::{collections::HashMap, collections::HashSet};

fn main() {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let mut grid: HashMap<(i32, i32), i8> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), char as i8 - 48);
        }
    }

    let res = grid
        .iter()
        .filter(|(_, &value)| value.eq(&0))
        .fold((0, 0), |acc, (pos, _)| {
            let res = solve(*pos, &grid, &mut HashSet::new());
            return (acc.0 + res.0, acc.1 + res.1);
        });

    println!("part1: {:? }", res.0);
    println!("part2: {:? }", res.1);
}

fn solve(
    cur: (i32, i32),
    grid: &HashMap<(i32, i32), i8>,
    seen: &mut HashSet<(i32, i32)>,
) -> (i32, i32) {
    let value: &i8 = grid.get(&cur).unwrap();
    if value.eq(&9) {
        if seen.contains(&cur) {
            return (0, 1);
        }

        seen.insert(cur);
        return (1, 1);
    }

    let mut sum: (i32, i32) = (0, 0);

    for (x, y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let next_pos = (cur.0 + x, cur.1 + y);

        let next = grid.get(&next_pos);
        if next.is_none() {
            continue;
        }

        let next_value = next.unwrap();

        if next_value - value != 1 {
            continue;
        }

        let res = solve(next_pos, grid, seen);

        sum.0 += res.0;
        sum.1 += res.1;
    }

    return sum;
}
