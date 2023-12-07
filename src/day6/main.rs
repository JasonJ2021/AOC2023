use std::f64::EPSILON;

fn parse_to_f64(s: &str) -> f64 {
    let v = s.split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>();
    let s = v.join("");
    s.parse::<f64>().unwrap()
}

fn parse_data(s: &str) -> Vec<usize> {
    s.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn compute(m: f64, n: f64) -> f64{
    let delta = ((m * m - 4_f64 * n) as f64).sqrt();
    let x_0 = ((m as f64 - delta) / 2_f64).ceil();
    let x_1 = ((m as f64 + delta) / 2_f64).floor();
    println!("x_0:{x_0}, x_1:{x_1}");
    println!("{}", x_1 - x_0 + 1.0);
    x_1 - x_0 + 1.0
}
fn main() {
    let input = std::fs::read("src/day6/input.txt").unwrap();
    let input = std::str::from_utf8(&input).unwrap();
    let lines = input.lines().take(2).collect::<Vec<_>>();
    println!("lines:{:?}", lines);
    let time = lines[0].strip_prefix("Time:").unwrap().trim();
    let distance = lines[1].strip_prefix("Distance:").unwrap().trim();
    let times = parse_data(time);
    let distances = parse_data(distance);
    println!("Time:{time:?}, distance:{distance:?}");
    let mut ans_part1 = 1_f64;
    for race_idx in 0..times.len() {
        let m = times[race_idx] as f64;
        let n = distances[race_idx] as f64 + 0.5;
        if m * m - 4_f64 * n < 0.0 {
            continue;
        }
        ans_part1 *= compute(m, n);
    }
    println!("ans_part1:{ans_part1}");
    let time = parse_to_f64(time);
    let distance = parse_to_f64(distance);
    println!("{time},{distance}");
    println!("ans_part2:{}", compute(time, distance));
}
