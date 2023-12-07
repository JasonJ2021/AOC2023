use core::fmt;
use std::{
    cmp::Ordering,
    collections::{hash_map, HashMap},
};

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    cards: &'a str,
    bid: usize,
    typ: usize,
    typ_wt_joker: usize,
}

fn sort_by_wo_joker(a: &Hand, b: &Hand) -> Ordering {
    a.typ
        .cmp(&b.typ)
        .then_with(|| cmp_str_priority_wo_joker(a.cards, b.cards))
}

fn sort_by_wt_joker(a: &Hand, b: &Hand) -> Ordering {
    a.typ_wt_joker
        .cmp(&b.typ_wt_joker)
        .then_with(|| cmp_str_priority_wt_joker(a.cards, b.cards))
}

fn cmp_str_priority_wt_joker(a: &str, b: &str) -> Ordering {
    let PRIORITY_MAP: HashMap<char, usize> = HashMap::from([
        ('J', 0),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
    for (a_c, b_c) in a.chars().zip(b.chars()) {
        let a_c_pri = PRIORITY_MAP.get(&a_c).unwrap();
        let b_c_pri = PRIORITY_MAP.get(&b_c).unwrap();
        if a_c_pri > b_c_pri {
            return Ordering::Greater;
        } else if a_c_pri < b_c_pri {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn cmp_str_priority_wo_joker(a: &str, b: &str) -> Ordering {
    let PRIORITY_MAP: HashMap<char, usize> = HashMap::from([
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('J', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);
    for (a_c, b_c) in a.chars().zip(b.chars()) {
        let a_c_pri = PRIORITY_MAP.get(&a_c).unwrap();
        let b_c_pri = PRIORITY_MAP.get(&b_c).unwrap();
        if a_c_pri > b_c_pri {
            return Ordering::Greater;
        } else if a_c_pri < b_c_pri {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

impl<'a> fmt::Display for Hand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cards:{}, bid:{}, typ:{}",
            self.cards, self.bid, self.typ
        )
    }
}

impl<'a> Hand<'a> {
    fn new(cards: &'a str, bid: usize) -> Self {
        let mut card_map = HashMap::new();
        for c in cards.chars() {
            if let Some(&freq) = card_map.get(&c) {
                card_map.insert(c, freq + 1);
            } else {
                card_map.insert(c, 1_usize);
            }
        }
        let mut five_kind = (false, 'a');
        let mut four_kind = (false, 'a');
        let mut triples = (false, 'a');
        // #pairs
        let mut pairs = Vec::new();
        for (&c, &freq) in &card_map {
            if freq == 5 {
                five_kind = (true, c);
            } else if freq == 4 {
                four_kind = (true, c);
            } else if freq == 3 {
                triples = (true, c);
            } else if freq == 2 {
                pairs.push(c);
            }
        }
        let typ: usize;
        let mut typ_joker = None;
        if five_kind.0 {
            typ = 7;
        } else if four_kind.0 {
            if four_kind.1 == 'J' || card_map.contains_key(&'J') {
                typ_joker = Some(7);
            }
            typ = 6;
        } else if triples.0 && pairs.len() == 1 {
            if (triples.1 == 'J' || pairs[0] == 'J') {
                typ_joker = Some(7);
            }
            typ = 5;
        } else if triples.0 {
            // JJJ +xy or xxx + Jy
            if triples.1 == 'J' || card_map.contains_key(&'J') {
                typ_joker = Some(6);
            }
            typ = 4;
        } else if pairs.len() == 2 {
            // JJ + XX + Y or XX + YY + J
            if pairs[0] == 'J' || pairs[1] == 'J' {
                typ_joker = Some(6);
            } else if card_map.contains_key(&'J') {
                typ_joker = Some(5);
            }
            typ = 3;
        } else if pairs.len() == 1 {
            // JJ + XYZ or XX JYZ
            if card_map.contains_key(&'J') {
                typ_joker = Some(4);
            }
            typ = 2;
        } else {
            // XYZKJ
            if card_map.contains_key(&'J') {
                typ_joker = Some(2);
            }
            typ = 1;
        }
        if typ_joker.is_none() {
            typ_joker = Some(typ);
        }
        Hand {
            cards,
            bid,
            typ: typ,
            typ_wt_joker: typ_joker.unwrap(),
        }
    }
}

fn main() {
    let input = std::fs::read("src/day7/input.txt").unwrap();
    let input = std::str::from_utf8(&input).unwrap();
    let mut hands = input
        .lines()
        .map(|line| {
            let v = line.split(" ").collect::<Vec<_>>();
            Hand::new(v[0], v[1].parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    hands.sort_by(sort_by_wo_joker);
    for hand in &hands {
        println!("{hand:?}");
    }
    let mut ans_part1 = 0;
    for (mut rank, hand) in hands.iter().enumerate() {
        rank = rank + 1;
        ans_part1 += rank * hand.bid;
    }
    println!("PART1 ANSWER:{}", ans_part1);
    let mut ans_part2 = 0;
    hands.sort_by(sort_by_wt_joker);
    for hand in &hands {
        println!("{hand:?}");
    }
    for (mut rank, hand) in hands.iter().enumerate() {
        rank = rank + 1;
        ans_part2 += rank * hand.bid;
    }
    println!("PART@ ANSWER:{}", ans_part2);
}
