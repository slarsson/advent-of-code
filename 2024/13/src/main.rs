fn main() {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    let numbers = input
        .split("\n\n")
        .map(|block| {
            block
                .split("\n")
                .map(|line| {
                    let matches = regex::Regex::new(r"\d+").unwrap();
                    return matches
                        .find_iter(line)
                        .map(|x| x.as_str().parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                })
                .collect::<Vec<Vec<i64>>>()
        })
        .collect::<Vec<Vec<Vec<i64>>>>();

    let games: Vec<Game> = numbers
        .iter()
        .map(|game| {
            return Game {
                a: (game[0][0], game[0][1]),
                b: (game[1][0], game[1][1]),
                res: (game[2][0], game[2][1]),
            };
        })
        .collect();

    println!("part1: {}", solve(games.clone(), 0, 100));
    println!("part2: {}", solve(games.clone(), 10000000000000, i64::MAX));
}

fn solve(games: Vec<Game>, mult: i64, limit: i64) -> i64 {
    let mut sum = 0;

    for game in games {
        let b = (game.a.0 * (game.res.1 + mult) - game.a.1 * (game.res.0 + mult)) as f64
            / (game.a.0 * game.b.1 - game.a.1 * game.b.0) as f64;

        let a = ((game.res.0 + mult) as f64 - game.b.0 as f64 * b) / game.a.0 as f64;

        if a < 0.0 || b < 0.0 {
            continue;
        }

        if a > limit as f64 || b > limit as f64 {
            continue;
        }

        if a != a.trunc() || b != b.trunc() {
            continue;
        }

        sum += 3 * a as i64 + b as i64;
    }

    return sum;
}

#[derive(Debug, Clone)]
struct Game {
    a: (i64, i64),
    b: (i64, i64),
    res: (i64, i64),
}
