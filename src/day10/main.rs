use std::collections::VecDeque;

fn is_valid(x: i32, y: i32, graph: &Vec<&str>) -> bool {
    x >= 0 && x < graph.len() as i32 && y >= 0 && y < graph[0].len() as i32
}

fn main() {
    let input = std::fs::read("src/day10/small.txt").unwrap();
    let input = std::str::from_utf8(&input).unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let mut marked = vec![vec![false; lines[0].len()]; lines.len()];
    let mut queue = VecDeque::new();
    let mut start = (0, 0);
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i].chars().nth(j).unwrap() == 'S' {
                start = (i, j);
            }
        }
    }
    marked[start.0][start.1] = true;
    // x, y, pipe char, dis
    // TODO: Refine this
    let x = start.0 as i32;
    let y = start.1 as i32 - 1;
    if is_valid(x, y, &lines) {
        let ch = lines[x as usize].chars().nth(y as usize).unwrap();
        if ch == '-' || ch == 'L' || ch == 'F' {
            marked[x as usize][y as usize] = true;
            queue.push_back((x, y, ch, 1));
        }
    }
    let x = start.0 as i32 - 1;
    let y = start.1 as i32;
    if is_valid(x, y, &lines) {
        let ch = lines[x as usize].chars().nth(y as usize).unwrap();
        if ch == '|' || ch == '7' || ch == 'F' {
            marked[x as usize][y as usize] = true;
            queue.push_back((x, y, ch, 1));
        }
    }
    let x = start.0 as i32;
    let y = start.1 as i32 + 1;
    if is_valid(x, y, &lines) {
        let ch = lines[x as usize].chars().nth(y as usize).unwrap();
        if ch == '-' || ch == 'J' || ch == '7' {
            marked[x as usize][y as usize] = true;
            queue.push_back((x, y, ch, 1));
        }
    }
    let x = start.0 as i32 + 1;
    let y = start.1 as i32;
    if is_valid(x, y, &lines) {
        let ch = lines[x as usize].chars().nth(y as usize).unwrap();
        if ch == '|' || ch == 'J' || ch == 'L' {
            marked[x as usize][y as usize] = true;
            queue.push_back((x, y, ch, 1));
        }
    }
    let mut max_dist = i32::MIN;
    while !queue.is_empty() {
        let loc = queue.pop_front().unwrap();
        if loc.2 == 'S' || loc.2 == '.' {
            continue;
        }
        max_dist = max_dist.max(loc.3);
        let next_loc = get_next_loc(loc.2, loc.0, loc.1);
        move_loc(
            next_loc.0,
            next_loc.1,
            next_loc.2,
            next_loc.3,
            loc.3,
            &lines,
            &mut marked,
            &mut queue,
        )
    }
    println!("max_dist:{max_dist}");
}

fn get_next_loc(ch: char, x: i32, y: i32) -> (i32, i32, i32, i32) {
    match ch {
        '|' => {
            let x_1 = x - 1;
            let y_1 = y;
            let x_2 = x + 1;
            let y_2 = y;
            (x_1, y_1, x_2, y_2)
        }
        '-' => {
            let x_1 = x;
            let y_1 = y - 1;
            let x_2 = x;
            let y_2 = y + 1;
            (x_1, y_1, x_2, y_2)
        }
        'L' => {
            let x_1 = x + 1;
            let y_1 = y;
            let x_2 = x;
            let y_2 = y + 1;
            (x_1, y_1, x_2, y_2)
        }
        'J' => {
            let x_1 = x - 1;
            let y_1 = y;
            let x_2 = x;
            let y_2 = y - 1;
            (x_1, y_1, x_2, y_2)
        }
        '7' => {
            let x_1 = x + 1;
            let y_1 = y;
            let x_2 = x;
            let y_2 = y - 1;
            (x_1, y_1, x_2, y_2)
        }
        'F' => {
            let x_1 = x + 1;
            let y_1 = y;
            let x_2 = x;
            let y_2 = y + 1;
            (x_1, y_1, x_2, y_2)
        }
        _ => unreachable!(),
    }
}
fn move_loc(
    x_1: i32,
    y_1: i32,
    x_2: i32,
    y_2: i32,
    dis: i32,
    graph: &Vec<&str>,
    marked: &mut Vec<Vec<bool>>,
    queue: &mut VecDeque<(i32, i32, char, i32)>,
) {
    if is_valid(x_1, y_1, graph) && !marked[x_1 as usize][y_1 as usize] {
        marked[x_1 as usize][y_1 as usize] = true;
        queue.push_back((
            x_1,
            y_1,
            graph[x_1 as usize].chars().nth(y_1 as usize).unwrap(),
            dis + 1,
        ));
    }
    if is_valid(x_2, y_2, graph) && !marked[x_2 as usize][y_2 as usize] {
        marked[x_2 as usize][y_2 as usize] = true;
        queue.push_back((
            x_2,
            y_2,
            graph[x_2 as usize].chars().nth(y_2 as usize).unwrap(),
            dis + 1,
        ));
    }
}
