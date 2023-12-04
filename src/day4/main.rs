use std::collections::HashSet;

fn main() {
    let input = std::fs::read("src/day4/input.txt").unwrap();
    let input = std::str::from_utf8(&input).unwrap();

    let cards = input
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split(": ").collect();
            v[1].trim()
        })
        .collect::<Vec<_>>();
    let cards_matches = cards
        .iter()
        .map(|&card| {
            let v: Vec<&str> = card.split('|').collect();
            let winning_nums: HashSet<i32> = v[0]
                .split_terminator(' ')
                .filter(|s| !s.is_empty())
                .map(|nums| nums.parse::<i32>().unwrap())
                .collect();
            let nums_we_have: Vec<i32> = v[1]
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|nums| nums.parse::<i32>().unwrap())
                .collect();
            let ans = nums_we_have
                .iter()
                .filter(|num| winning_nums.contains(num))
                .count();
            ans
        })
        .collect::<Vec<_>>();
    let cards_point = cards_matches
        .iter()
        .map(|&num| {
            if num == (0 as usize) {
                0 as usize
            } else {
                usize::pow(2, (num - 1) as u32)
            }
        })
        .collect::<Vec<_>>();
    let ans_part1: usize = cards_point.iter().sum();
    println!("PART1 ANSWER:{}", ans_part1);
    let mut cards_cnt = vec![1; cards_point.len()];
    println!("points:{:?}", cards_point);
    for i in 0..cards_cnt.len() {
        let point = cards_matches[i];
        let cnt = cards_cnt[i];
        for j in i + 1..cards.len().min(i + point + 1) {
            println!("card {} add cnt {}", j, cnt);
            cards_cnt[j] += cnt;
        }
    }
    let ans_part2: usize = cards_cnt.iter().sum();
    println!("{:?}", cards_cnt);
    println!("PART2 ANSWER:{ans_part2}");
}
