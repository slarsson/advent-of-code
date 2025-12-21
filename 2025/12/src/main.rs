fn main() {
    let (_, rows) = load();

    let res = rows.iter().fold(0, |sum, row| {
        let package_area = row.2.iter().fold(0, |acc, v| acc + v * 9);
        if package_area <= row.0 * row.1 {
            return sum + 1;
        }
        return sum;
    });

    println!("result: {}", res);
}

fn load() -> (Vec<Vec<(usize, usize)>>, Vec<(usize, usize, Vec<usize>)>) {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let sections: Vec<&str> = input.split("\n\n").collect();

    let packages: Vec<Vec<(usize, usize)>> = sections[0..sections.len() - 1]
        .iter()
        .map(|v| {
            let lines: Vec<&str> = v.lines().skip(1).collect();
            let mut tiles = Vec::new();

            for (y, line) in lines.iter().enumerate() {
                for (x, ch) in line.chars().enumerate() {
                    if ch.eq(&'#') {
                        tiles.push((x, y));
                    }
                }
            }

            return tiles;
        })
        .collect();

    let mut rows: Vec<(usize, usize, Vec<usize>)> = Vec::new();
    for line in sections.last().unwrap().lines() {
        let parts: Vec<&str> = line.split(":").collect();
        assert!(parts.len() == 2);

        let dimensions: Vec<usize> = parts[0]
            .split("x")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let counts: Vec<usize> = parts[1]
            .split(" ")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        rows.push((dimensions[0], dimensions[1], counts));
    }

    return (packages, rows);
}
