use core::panic;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    str::from_utf8,
};

fn calc_calibration(line: &str) -> i32 {
    let mut first_digit = -1;
    let mut last_digit = -1;
    for (_, c) in line.chars().enumerate() {
        if c.is_numeric() {
            if first_digit == -1 {
                first_digit = c.to_digit(10).unwrap() as i32;
            }
            last_digit = c.to_digit(10).unwrap() as i32;
        }
    }
    let calibration = first_digit * 10 + last_digit;
    calibration
}

fn subst_word(mut line: String) -> String {
    let mut pos = 0;
    let line = line.as_bytes();
    let substs = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut new_line = String::new();
    while pos < line.len() {
        let starts_from = &line[pos..];
        let mut flag = false;
        for (from, to) in substs {
            if starts_from.starts_with(from.as_bytes()) {
                new_line.push_str(to);
                flag = true;
                break;
            }
        }
        if !flag {
            let ch: char = starts_from[0].into();
            new_line.push(ch);
        }
        pos += 1;
    }
    new_line
}

fn main() {
    let filename = "src/day1/input.txt";
    let file = Path::new(filename);
    let mut sum = 0;
    if let Ok(lines) = read_lines(file) {
        for (_line_idx, line) in lines.enumerate() {
            if let Ok(line) = line {
                sum += calc_calibration(line.as_str());
            }
        }
    }
    println!("Part1 sum: {sum}");
    sum = 0;
    if let Ok(lines) = read_lines(file) {
        for (_line_idx, line) in lines.enumerate() {
            if let Ok(mut line) = line {
                line = subst_word(line);
                let num = calc_calibration(line.as_str());
                // println!("num: {num}");
                sum += num;
            }
        }
    }
    println!("Part2 sum: {sum}");
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
