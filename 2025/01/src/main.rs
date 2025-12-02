fn main() {
    let rotations = load();
    println!("part1: {:?}", part1(rotations.clone()));
    println!("part2: {:?}", part2(rotations.clone()));
}

fn load() -> Vec<(i32, i32)> {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let rotations: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let x = match line.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => panic!("nehe"),
            };

            return (x, line[1..].parse::<i32>().unwrap());
        })
        .collect();

    return rotations;
}

fn part1(input: Vec<(i32, i32)>) -> i32 {
    let mut cursor = 50;
    let mut count = 0;

    for (dir, step) in input {
        cursor += dir * step;
        cursor = cursor % 100;
        if cursor == 0 {
            count += 1;
        }
    }

    return count;
}

fn part2(input: Vec<(i32, i32)>) -> i32 {
    let mut cursor = 50;
    let mut count = 0;

    for (dir, step) in input {
        count += step / 100;

        let starts_on_zero = cursor == 0;

        cursor += dir * (step % 100);

        if cursor == 0 {
            count += 1;
        } else if cursor < 0 {
            if !starts_on_zero {
                count += 1;
            }
            cursor += 100;
        } else if cursor > 99 {
            count += 1;
            cursor -= 100;
        }
    }

    return count;
}
