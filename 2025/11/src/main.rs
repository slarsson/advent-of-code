use std::collections::HashMap;

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> usize {
    let graph = load();

    return solve(
        String::from("you"),
        &String::from("out"),
        &graph,
        &mut HashMap::new(),
    );
}

fn part2() -> usize {
    let inputs = load();

    let dac_to_fft = solve(
        String::from("dac"),
        &String::from("fft"),
        &inputs,
        &mut HashMap::new(),
    );

    let fft_to_dac = solve(
        String::from("fft"),
        &String::from("dac"),
        &inputs,
        &mut HashMap::new(),
    );

    let (first, second, middle) = match (dac_to_fft == 0, fft_to_dac == 0) {
        (true, false) => (String::from("fft"), String::from("dac"), fft_to_dac),
        (false, true) => (String::from("dac"), String::from("fft"), dac_to_fft),
        _ => panic!("nehe.."),
    };

    let start = solve(String::from("svr"), &first, &inputs, &mut HashMap::new());
    let end = solve(second, &String::from("out"), &inputs, &mut HashMap::new());

    return start * middle * end;
}

fn solve(
    from: String,
    target: &String,
    graph: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if from.eq(target) {
        return 1;
    }

    if let Some(sum) = cache.get(&from) {
        return *sum;
    }

    let mut sum = 0;
    if let Some(connections) = graph.get(&from) {
        for to in connections {
            sum += solve(to.clone(), target, graph, cache);
        }
    }

    cache.insert(from, sum);

    return sum;
}

fn load() -> HashMap<String, Vec<String>> {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    return input
        .lines()
        .map(|line| {
            let strings: Vec<String> = line
                .split(' ')
                .map(|s| s.replace(':', "").to_string())
                .collect();

            return (
                strings.first().unwrap().clone(),
                strings[1..].iter().map(|s| s.to_string()).collect(),
            );
        })
        .collect();
}
