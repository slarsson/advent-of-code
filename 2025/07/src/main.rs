use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> i32 {
    let (start, max_y, grid) = load();

    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut count = 0;

    queue.push_back(start);

    loop {
        let head = if let Some(v) = queue.pop_front() {
            v
        } else {
            break;
        };

        let next = (head.0, head.1 + 1);

        if next.1 > max_y {
            continue;
        }

        if seen.get(&next).is_some() {
            continue;
        }

        seen.insert(next);

        if grid.get(&next).is_some() {
            queue.push_back((next.0 - 1, next.1));
            queue.push_back((next.0 + 1, next.1));
            count += 1;
        } else {
            queue.push_back(next);
        }
    }

    return count;
}

fn part2() -> i64 {
    let (start, max_y, grid) = load();
    let mut cache: HashMap<(i32, i32), i64> = HashMap::new();
    return solve_part2(start, &max_y, &grid, &mut cache);
}

fn solve_part2(
    cur: (i32, i32),
    max_y: &i32,
    grid: &HashSet<(i32, i32)>,
    cache: &mut HashMap<(i32, i32), i64>,
) -> i64 {
    if let Some(cached_value) = cache.get(&cur) {
        return *cached_value;
    }

    let mut sum = 0i64;
    if cur.1 > *max_y {
        return 1;
    }

    if grid.get(&cur).is_some() {
        sum += solve_part2((cur.0 - 1, cur.1), max_y, grid, cache);
        sum += solve_part2((cur.0 + 1, cur.1), max_y, grid, cache);
    } else {
        sum += solve_part2((cur.0, cur.1 + 1), max_y, grid, cache);
    }

    cache.insert(cur, sum);

    return sum;
}

fn load() -> ((i32, i32), i32, HashSet<(i32, i32)>) {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let grid: HashSet<(i32, i32)> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| return c.eq(&'^'))
                .map(|(x, _)| (x as i32, y as i32))
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .collect();

    let start_x = input.lines().next().unwrap().len() / 2;

    let max_y = input.lines().count() as i32 - 1;

    return ((start_x as i32, 0), max_y, grid);
}
