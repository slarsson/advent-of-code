fn main() {
    let numbers = load();

    println!(
        "part1: {}",
        numbers
            .iter()
            .map(|row| solve(row, 2))
            .fold(0u64, |acc, v| acc + v)
    );

    println!(
        "part2: {}",
        numbers
            .iter()
            .map(|row| solve(row, 12))
            .fold(0u64, |acc, v| acc + v)
    );
}

fn load() -> Vec<Vec<u32>> {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    return input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
}

fn solve(numbers: &Vec<u32>, window_size: usize) -> u64 {
    let mut w = Window::new(window_size);
    w.scan(numbers);
    return w.sum();
}

struct Window {
    size: usize,
    values: Vec<u64>,
}

impl Window {
    pub fn new(size: usize) -> Window {
        return Window {
            size: size,
            values: Vec::with_capacity(size),
        };
    }

    pub fn scan(&mut self, values: &Vec<u32>) {
        values.iter().rev().for_each(|&v| self.scan_end(v as u64));
    }

    fn scan_end(&mut self, value: u64) {
        if self.values.len() < self.size {
            self.values.insert(0, value);
            return;
        }

        let head = self.values.first().unwrap();

        if head > &value {
            return;
        }

        if let Some(i) = self
            .values
            .windows(2)
            .enumerate()
            .find(|(_, w)| w[1] > w[0])
            .map(|(i, _)| i)
        {
            self.values.remove(i);
        } else {
            self.values.pop();
        }

        self.values.insert(0, value);
    }

    pub fn sum(&self) -> u64 {
        return self
            .values
            .iter()
            .rev()
            .fold((0, 1), |(sum, incr), &v| (sum + v * incr, incr * 10))
            .0;
    }
}
