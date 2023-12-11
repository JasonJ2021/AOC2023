use core::panic;

fn main() {
    let input = std::fs::read("src/day9/input.txt").unwrap();
    let input = std::str::from_utf8(&input).unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let mut ans_part1 = 0;
    let mut ans_part2 = 0;
    for line in lines {
        let nums = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let extra_value_right= find_extra_right(&nums);
        let extra_value_left = find_extra_left(&nums);
        ans_part1 += extra_value_right;
        ans_part2 += extra_value_left
    }
    println!("Answer part1:{ans_part1}");
    println!("Answer part2:{ans_part2}");
}

fn find_extra_right(nums: &Vec<i32>) -> i32 {
    if nums.is_empty() {
        panic!("Should not reach here");
    }
    if nums.iter().min() == nums.iter().max() {
        return nums[0];
    }
    let mut next = Vec::new();
    for i in 1..nums.len() {
        next.push(nums[i] - nums[i - 1]);
    }
    return nums[nums.len() - 1] + find_extra_right(&next);
}

fn find_extra_left(nums: &Vec<i32>) -> i32 {
    if nums.is_empty() {
        panic!("Should not reach here");
    }
    if nums.iter().min() == nums.iter().max() {
        return nums[0];
    }
    let mut next = Vec::new();
    for i in 1..nums.len() {
        next.push(nums[i] - nums[i - 1]);
    }
    return nums[0] - find_extra_left(&next);
}
