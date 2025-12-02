fn main() {
    let ranges = load();
    println!("part1: {:?}", part1(ranges.clone()));
    println!("part2: {:?}", part2(ranges.clone()));
}

fn load() -> Vec<(i64, i64)> {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    return input
        .split(",")
        .map(|range| {
            let bounds = range
                .split("-")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (bounds[0], bounds[1])
        })
        .collect();
}

fn part1(ranges: Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;

    for (start, end) in ranges {
        let mut cursor = start;
        loop {
            if cursor > end {
                break;
            }

            let number_of_digits = cursor.ilog10() + 1;

            let middle = number_of_digits / 2;

            let lhs = cursor / 10i64.pow(middle);
            let rhs = cursor % 10i64.pow(middle);

            if lhs == rhs {
                sum += cursor;
            }

            cursor += 1;
        }
    }

    return sum;
}

fn part2(ranges: Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;

    for (start, end) in ranges {
        let mut cursor = end;
        loop {
            if cursor < start {
                break;
            }

            let number_of_digits = cursor.ilog10() + 1;
            let mut window_size = number_of_digits / 2;

            loop {
                if window_size < 1 {
                    break;
                }

                if valid_sequence(cursor, window_size) {
                    sum += cursor;
                    break;
                }

                window_size -= 1;
            }

            cursor -= 1;
        }
    }

    return sum;
}

fn valid_sequence(value: i64, window_size: u32) -> bool {
    let number_of_digits = value.ilog10() + 1;

    if number_of_digits % window_size != 0 {
        return false;
    }

    let mut cursor = value;
    let mut prev_tail: Option<i64> = None;

    loop {
        if cursor == 0 {
            break;
        }

        let new_tail = cursor % 10i64.pow(window_size);
        cursor = cursor / 10i64.pow(window_size);

        if let Some(tail) = prev_tail {
            if new_tail != tail {
                return false;
            }
        } else {
            prev_tail = Some(new_tail);
        }
    }

    return true;
}
