use std::collections::HashMap;

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

#[derive(Debug, PartialEq)]
enum Operation {
    ADD,
    MULT,
}

fn load() -> (Vec<Vec<i64>>, Vec<Operation>) {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let lines: Vec<&str> = input.split("\n").collect();

    let numbers: Vec<Vec<i64>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect();

            return numbers;
        })
        .collect();

    let operations: Vec<Operation> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|v| match v {
            "+" => Operation::ADD,
            "*" => Operation::MULT,
            _ => panic!("nehe"),
        })
        .collect();

    return (numbers, operations);
}

fn load_grid() -> HashMap<(usize, usize), i32> {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let lines: Vec<&str> = input.split("\n").collect();

    let grid: HashMap<(usize, usize), i32> = lines[..lines.len() - 1]
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(move |(x, ch)| {
                let v = if ch.is_whitespace() {
                    -1
                } else {
                    ch.to_digit(10).unwrap() as i32
                };
                return ((x, y), v);
            })
        })
        .flatten()
        .collect();

    return grid;
}

fn part1() -> i64 {
    let (numbers, operations) = load();
    let size = numbers.get(0).unwrap().len();

    return (0..size).fold(0, |acc, idx| {
        let op = operations.get(idx).unwrap();
        let start = if *op == Operation::ADD { 0 } else { 1 };
        let sum = numbers.iter().fold(start, |tot, v| match op {
            Operation::ADD => tot + v.get(idx).unwrap(),
            Operation::MULT => tot * v.get(idx).unwrap(),
        });
        return acc + sum;
    });
}

fn part2() -> i64 {
    let (_, operations) = load();
    let grid = load_grid();

    let mut x_cur = 0;
    let mut sum = 0i64;

    for op in operations {
        let mut number = if op == Operation::ADD { 0 } else { 1 };

        loop {
            let mut y_cur = 0;
            let mut acc = 0i64;

            loop {
                let value = if let Some(v) = grid.get(&(x_cur, y_cur)) {
                    v
                } else {
                    break;
                };

                if *value > 0 {
                    acc = acc * 10 + (*value) as i64;
                }

                y_cur += 1;
            }

            x_cur += 1;

            if acc == 0 {
                break;
            }

            match op {
                Operation::ADD => number += acc,
                Operation::MULT => number *= acc,
            }
        }

        sum += number;
    }

    return sum;
}
