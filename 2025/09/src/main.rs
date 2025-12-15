fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn load() -> Vec<(i64, i64)> {
    let path = std::env::current_dir().unwrap().as_path().join("input.txt");
    let input: String = std::fs::read_to_string(path).expect("no file 4 you");

    return input
        .lines()
        .map(|line| {
            let coord: Vec<i64> = line.split(',').map(|v| v.parse::<i64>().unwrap()).collect();
            assert!(coord.len() == 2);
            return (coord[0], coord[1]);
        })
        .collect();
}

fn part1() -> i64 {
    let coords = load();

    let mut max = 0;

    for i in 0..coords.len() - 1 {
        for j in (i + 1)..coords.len() {
            let a = coords[i];
            let b = coords[j];

            let area = ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1);

            if area > max {
                max = area;
            }
        }
    }

    return max;
}

fn part2() -> i64 {
    let coords = load();
    let polygon = make_polygon(&coords);

    let mut max = 0;

    for i in 0..coords.len() - 1 {
        'outer: for j in (i + 1)..coords.len() {
            let a = coords[i];
            let b = coords[j];

            let area = ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1);
            if area <= max {
                continue;
            }

            let rect = rectangle(a, b);

            for v in &rect {
                if !point_in_polygon(v.0, &polygon) {
                    continue 'outer;
                }
            }

            for line in rect {
                if line.0.0 == line.1.0 {
                    let x = line.0.0;
                    let y_max = line.0.1.max(line.1.1);
                    let y_min = line.0.1.min(line.1.1);

                    for v in y_min + 1..y_max {
                        if !point_in_polygon((x, v), &polygon) {
                            continue 'outer;
                        }
                    }
                } else if line.0.1 == line.1.1 {
                    let y = line.0.1;
                    let x_max = line.0.0.max(line.1.0);
                    let x_min = line.0.0.min(line.1.0);

                    for v in x_min + 1..x_max {
                        if !point_in_polygon((v, y), &polygon) {
                            continue 'outer;
                        }
                    }
                }
            }

            if area > max {
                max = area;
            }
        }
    }

    return max;
}

fn make_polygon(v: &Vec<(i64, i64)>) -> Vec<((i64, i64), (i64, i64))> {
    let mut out: Vec<((i64, i64), (i64, i64))> = v.windows(2).map(|v| (v[0], v[1])).collect();
    out.push((*v.last().unwrap(), *v.first().unwrap()));
    return out;
}

// https://observablehq.com/@tmcw/understanding-point-in-polygon
fn point_in_polygon(p: (i64, i64), polygon: &Vec<((i64, i64), (i64, i64))>) -> bool {
    let mut hits = 0;

    let x0 = p.0;
    let y0 = p.1;

    for ((x1, y1), (x2, y2)) in polygon {
        let cross = (x2 - x1) * (y0 - y1) - (y2 - y1) * (x0 - x1);
        if cross != 0 {
            continue;
        }

        let x_min = *x1.min(x2);
        let x_max = *x1.max(x2);

        let y_min = *y1.min(y2);
        let y_max = *y1.max(y2);

        if x0 >= x_min && x0 <= x_max && y0 >= y_min && y0 <= y_max {
            return true;
        }
    }

    for ((x1, y1), (x2, y2)) in polygon {
        if (y0 < *y1) != (y0 < *y2) {
            if (x0 as f64)
                < (*x1 as f64) + (((y0 - y1) as f64) / ((y2 - y1) as f64)) * ((x2 - x1) as f64)
            {
                hits += 1;
            }
        }
    }

    return hits % 2 == 1;
}

fn rectangle(a: (i64, i64), b: (i64, i64)) -> Vec<((i64, i64), (i64, i64))> {
    let x_min = a.0.min(b.0);
    let x_max = a.0.max(b.0);

    let y_min = a.1.min(b.1);
    let y_max = a.1.max(b.1);

    return vec![
        ((x_min, y_min), (x_min, y_max)),
        ((x_min, y_max), (x_max, y_max)),
        ((x_max, y_max), (x_max, y_min)),
        ((x_max, y_min), (x_min, y_min)),
    ];
}
