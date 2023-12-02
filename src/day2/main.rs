use core::panic;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    str::from_utf8,
};

#[derive(Debug)]
struct game {
    id: usize,
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

fn main() {
    let filename = "src/day2/input.txt";
    let file = Path::new(filename);
    let RED = 12;
    let GREEN = 13;
    let BLUE = 14;
    if let Ok(lines) = read_lines(file) {
        // I use streams in rust awfully....
        // FIXME: rebellish this code
        let res = lines
            .filter_map(Result::ok)
            .map(|line| {
                let pos = line.find(":").unwrap();
                let game_id = line[5..pos]
                    .parse::<usize>()
                    .expect("Parsing a number error");
                let data = &line[pos + 2..];
                let mut max_red = 0;
                let mut max_green = 0;
                let mut max_blue = 0;
                let info = data
                    .split("; ")
                    .map(|s| {
                        let v = s.split(", ").collect::<Vec<_>>();
                        for s in v {
                            let v = s.split(' ').collect::<Vec<_>>();
                            let num = v[0].parse::<usize>().expect("Parsing usize failed");
                            match v[1] {
                                "red" => max_red = std::cmp::max(max_red, num),
                                "green" => max_green = std::cmp::max(max_green, num),
                                "blue" => max_blue = std::cmp::max(max_blue, num),
                                _ => unreachable!(),
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                game {
                    id: game_id,
                    max_red: max_red,
                    max_green: max_green,
                    max_blue: max_blue,
                }
            })
            .collect::<Vec<_>>();
        let sum_part1: usize = res
            .iter()
            .filter(|&game| {
                println!("game:{:?}", game);
                game.max_red <= RED && game.max_green <= GREEN && game.max_blue <= BLUE
            })
            .map(|game| game.id)
            .sum();
        println!("Part1 sum = {sum_part1}");
        let sum_part2: usize = res
            .iter()
            .map(|game| game.max_red * game.max_blue * game.max_green)
            .sum();
        println!("Part2 sum = {sum_part2}");
    }
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
