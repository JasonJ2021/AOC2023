use gcd::Gcd;
use std::{
    collections::{btree_map::Entry, HashMap},
    hash::Hash,
    sync::Arc,
};

fn is_end(input: &Vec<&str>) -> bool {
    for elem in input {
        if !elem.ends_with('Z') {
            return false;
        }
    }
    true
}

fn compute_steps_part1<'a>(
    mut cur: &'a str,
    commands: &str,
    maps: &HashMap<&str, (&'a str, &'a str)>,
) -> usize {
    let mut steps = 0;
    while cur != "ZZZ" {
        for i in 0..commands.len() {
            if cur == "ZZZ" {
                break;
            }
            let direction = commands.chars().nth(i).unwrap();
            if direction == 'L' {
                cur = maps[cur].0;
            } else {
                cur = maps[cur].1;
            }
            steps += 1;
        }
    }
    steps
}

fn compute_steps_part2<'a>(
    mut cur: &'a str,
    commands: &str,
    maps: &HashMap<&str, (&'a str, &'a str)>,
) -> usize {
    let mut steps = 0;
    while !cur.ends_with('Z') {
        for i in 0..commands.len() {
            if cur.ends_with('Z') {
                break;
            }
            let direction = commands.chars().nth(i).unwrap();
            if direction == 'L' {
                cur = maps[cur].0;
            } else {
                cur = maps[cur].1;
            }
            steps += 1;
        }
    }
    steps
}

fn main() {
    println!("Hello, world");
    let input = std::fs::read("src/day8/input.txt").unwrap();
    let input = std::str::from_utf8(&input).unwrap();
    let lines = input.lines().filter(|s| !s.is_empty()).collect::<Vec<_>>();
    println!("{lines:?}");
    let mut commands = "commands";
    let mut maps: HashMap<&str, (&str, &str)> = HashMap::new();
    for (idx, &line) in lines.iter().enumerate() {
        if idx == 0 {
            commands = line;
        } else {
            let map_elem = line
                .split('=')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let from = map_elem[0].trim_end();
            let to = map_elem[1]
                .trim()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(", ")
                .collect::<Vec<_>>();
            let to = (to[0], to[1]);
            maps.insert(from, to);
        }
    }
    let mut steps = 0_usize;
    let mut cur = "AAA";
    while cur != "ZZZ" {
        for i in 0..commands.len() {
            if cur == "ZZZ" {
                break;
            }
            let direction = commands.chars().nth(i).unwrap();
            if direction == 'L' {
                cur = maps[cur].0;
            } else {
                cur = maps[cur].1;
            }
            steps += 1;
        }
    }
    println!("steps:{steps}");

    // PART2
    steps = 0;
    let entries = maps
        .keys()
        .map(|&s| {
            if s.ends_with('A') {
                return s;
            }
            ""
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    // println!("{entries:?}");
    let steps = entries
        .iter()
        .map(|entry| compute_steps_part2(entry, commands, &maps))
        .collect::<Vec<_>>();
    // multiple(steps) / gcd(steps)
    let mut gcd: usize = steps[0].gcd(steps[1]);
    let mut multiple = steps[0] * steps[1];
    let mut ans =  multiple / gcd;
    for i in 2..steps.len(){
        multiple = ans * steps[i];
        gcd = ans.gcd(steps[i]);
        ans = multiple / gcd;
    }
    println!("Part2:{}",ans);
}
