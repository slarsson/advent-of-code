fn main() {
    let (ranges, values) = load();

    println!(
        "part1: {}",
        values
            .iter()
            .filter(|v| ranges
                .iter()
                .find(|(low, high)| v >= &low && v <= &high)
                .is_some())
            .count()
    );

    println!("part2: {}", part2(&ranges));
}

fn load() -> (Vec<(i64, i64)>, Vec<i64>) {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let sections: Vec<&str> = input.split("\n\n").collect();
    assert!(sections.len() == 2);

    let ranges: Vec<(i64, i64)> = sections
        .get(0)
        .unwrap()
        .lines()
        .filter(|line| !line.eq(&""))
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            return (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap());
        })
        .collect();

    let numbers: Vec<i64> = sections
        .get(1)
        .unwrap()
        .lines()
        .filter(|line| !line.eq(&""))
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    (ranges, numbers)
}

fn part2(ranges: &Vec<(i64, i64)>) -> i64 {
    let mut state = ranges.clone();
    let mut unique: Vec<(i64, i64)> = Vec::new();

    for i in 0..state.len() {
        let (a0, a1) = state[i];
        let mut is_unique = true;

        for j in (i + 1)..state.len() {
            let (b0, b1) = state[j];

            if a0 >= b0 && a1 <= b1 {
                is_unique = false;
                continue;
            }

            let a_overlap_left = a0 <= b0 && a1 >= b0;
            let a_overlap_right = a0 <= b1 && a1 >= b1;

            if a_overlap_left || a_overlap_right {
                is_unique = false;
                state[j] = (a0.min(b0), a1.max(b1))
            }
        }

        if is_unique {
            unique.push((a0, a1));
        }
    }

    return unique
        .iter()
        .fold(0, |acc, (from, to)| acc + (to - from + 1));
}
