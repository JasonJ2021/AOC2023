use std::{
    cmp::Ordering,
    collections::{binary_heap, BTreeSet, BinaryHeap},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Map {
    des_rng_start: usize,
    src_rng_start: usize,
    rng_len: usize,
}
impl Map {
    fn new(des_rng_start: usize, src_rng_start: usize, rng_len: usize) -> Self {
        Map {
            des_rng_start,
            src_rng_start,
            rng_len,
        }
    }
    fn from_str(s: &str) -> Self {
        let nums = s
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Map::new(nums[0], nums[1], nums[2])
    }
}
enum State {
    SeedToSoil = 0,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemp,
    TempToHumid,
    HumidToLoc,
}
impl Ord for Map {
    fn cmp(&self, other: &Self) -> Ordering {
        self.src_rng_start.cmp(&other.src_rng_start)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        other.from.cmp(&self.from)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Range {
    from: usize,
    len: usize,
}
impl Range {
    fn new(from: usize, len: usize) -> Self {
        Self { from, len }
    }
}
fn main() {
    let input = std::fs::read("src/day5/input.txt").unwrap();
    let input = std::str::from_utf8(&input).unwrap();
    let mut seeds = vec![];
    let mut State: Option<State> = None;
    let mut maps: Vec<BTreeSet<Map>> = vec![BTreeSet::<Map>::new(); 7];
    input
        .lines()
        .map(|mut line| {
            line = line.trim();
            if line.starts_with("seeds:") {
                let line = line.strip_prefix("seeds:").unwrap().trim();
                seeds = line
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
            } else if line.starts_with("seed-to-soil map:") {
                State = Some(State::SeedToSoil);
                return;
            } else if line.starts_with("soil-to-fertilizer map:") {
                State = Some(State::SoilToFertilizer);
                return;
            } else if line.starts_with("fertilizer-to-water map:") {
                State = Some(State::FertilizerToWater);
                return;
            } else if line.starts_with("water-to-light map:") {
                State = Some(State::WaterToLight);
                return;
            } else if line.starts_with("light-to-temperature map:") {
                State = Some(State::LightToTemp);
                return;
            } else if line.starts_with("temperature-to-humidity map:") {
                State = Some(State::TempToHumid);
                return;
            } else if line.starts_with("humidity-to-location map:") {
                State = Some(State::HumidToLoc);
                return;
            } else if line.is_empty() {
                return;
            }

            match State {
                None => {}
                Some(State::SeedToSoil) => {
                    maps[State::SeedToSoil as usize].insert(Map::from_str(line));
                }
                Some(State::SoilToFertilizer) => {
                    maps[State::SoilToFertilizer as usize].insert(Map::from_str(line));
                }
                Some(State::FertilizerToWater) => {
                    maps[State::FertilizerToWater as usize].insert(Map::from_str(line));
                }
                Some(State::WaterToLight) => {
                    maps[State::WaterToLight as usize].insert(Map::from_str(line));
                }
                Some(State::LightToTemp) => {
                    maps[State::LightToTemp as usize].insert(Map::from_str(line));
                }
                Some(State::TempToHumid) => {
                    maps[State::TempToHumid as usize].insert(Map::from_str(line));
                }
                Some(State::HumidToLoc) => {
                    maps[State::HumidToLoc as usize].insert(Map::from_str(line));
                }
            }
        })
        .collect::<Vec<_>>();
    let mut min_location = usize::MAX;
    // println!("{:?}",maps);
    println!("{}", seeds.len());
    for (seed) in seeds.iter() {
        let mut seed = *seed;
        for map in maps.iter() {
            for item in map {
                if seed >= item.src_rng_start && seed < item.src_rng_start + item.rng_len {
                    // println!("{},{},{}", item.des_rng_start,item.src_rng_start,item.rng_len);
                    let new_seed = seed - item.src_rng_start + item.des_rng_start;
                    // println!("from {seed} to {new_seed}");
                    seed = new_seed;
                    break;
                }
            }
        }
        min_location = min_location.min(seed);
    }
    println!("Part1 Answer: {min_location}");
    let line = input.lines().next().unwrap();
    line.strip_prefix("seeds:").unwrap().trim();
    let mut seeds_pairs: Vec<(usize, usize)> = Vec::new();
    for idx in (0..seeds.len()).step_by(2) {
        seeds_pairs.push((seeds[idx], seeds[idx + 1]));
    }
    // println!("{:?}", seeds_pairs);
    // println!("{:?}", maps);
    let mut lowest_loc = usize::MAX;
    // TODO!!!
    for (seed_from, seed_len) in seeds_pairs {
        // split the range
        let mut seeds_range_prev = BinaryHeap::new();
        seeds_range_prev.push(Range::new(seed_from, seed_len));
        for (idx, map) in maps.iter().enumerate() {
            let mut seeds_range_next = BinaryHeap::new();
            for item in map {
                while let Some(range) = seeds_range_prev.peek() {
                    // if range.from == 77 {
                    //     println!("{:?}", item);
                    //     println!("{:?}", range);
                    // }
                    if range.from >= item.src_rng_start
                        && range.from < item.src_rng_start + item.rng_len
                    {
                        // this range is overlapping with item
                        let range = seeds_range_prev.pop().unwrap();
                        let range_end = range.from + range.len - 1;
                        let range_from_new = item.des_rng_start + range.from - item.src_rng_start;
                        if range_end >= item.src_rng_start
                            && range_end < item.src_rng_start + item.rng_len
                        {
                            // this range is in item
                            seeds_range_next.push(Range::new(range_from_new, range.len));
                        } else {
                            //    |range_start.... range_end|
                            // |item_start.... item_end|
                            let in_len = item.src_rng_start + item.rng_len - range.from;
                            let remain_len = range.len - in_len;
                            seeds_range_next.push(Range::new(range_from_new, in_len));
                            if remain_len > 0 {
                                seeds_range_prev.push(Range::new(range.from + in_len, remain_len));
                            }
                        }
                    } else {
                        // this range is not overlapping with item
                        break;
                    }
                }
            }
            for range in &seeds_range_prev {
                seeds_range_next.push(range.clone());
            }
            // for range in &seeds_range_next {
            //     print!("({},{});", range.from, range.len);
            // }
            // println!();
            seeds_range_prev = seeds_range_next;
        }
        // println!("Final:");
        if let Some(range) = seeds_range_prev.peek() {
            lowest_loc = lowest_loc.min(range.from);
        }
        // for range in &seeds_range_prev {
        //     print!("({},{});", range.from, range.len);
        // }
        // println!();
        // println!("=======")
    }
    println!("PART2 ANSWER:{}", lowest_loc);
}
