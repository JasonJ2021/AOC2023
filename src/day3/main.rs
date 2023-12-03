use core::num;

fn part2(input: &str) {
    let mut graph = input
        .lines()
        .map(|line| line.to_owned().as_bytes().to_vec())
        .collect::<Vec<_>>();
    let m = graph.len();
    let n = graph[0].len();
    let mut v = Vec::new();
    // PART2
    for i in 0..m {
        for j in 0..n {
            if graph[i][j] == b'*' {
                let mut num_count = 0;
                let mut mult = 1;
                if j > 0 && graph[i][j - 1].is_ascii_digit() {
                    num_count += 1;
                    mult *= span_num(&graph, i, j - 1);
                }
                if j > 0 && i > 0 && graph[i - 1][j - 1].is_ascii_digit() {
                    num_count += 1;
                    mult *= span_num(&graph, i - 1, j - 1);
                }
                if i > 0 {
                    // top
                    if graph[i - 1][j].is_ascii_digit()
                        && ((j > 0 && !graph[i - 1][j - 1].is_ascii_digit()) || j == 0)
                    {
                        num_count += 1;
                        mult *= span_num(&graph, i - 1, j);
                    }
                }
                if i > 0 && j + 1 < n {
                    // top right
                    if graph[i - 1][j + 1].is_ascii_digit() && !graph[i - 1][j].is_ascii_digit() {
                        num_count += 1;
                        mult *= span_num(&graph, i - 1, j + 1);
                    }
                }
                if j + 1 < n && graph[i][j + 1].is_ascii_digit() {
                    // right
                    num_count += 1;
                    mult *= span_num(&graph, i, j + 1);
                }
                if i + 1 < m && j > 0 && graph[i + 1][j - 1].is_ascii_digit() {
                    // bottom left
                    num_count += 1;
                    mult *= span_num(&graph, i + 1, j - 1);
                }
                if i + 1 < m && graph[i + 1][j].is_ascii_digit() {
                    // bottom
                    if j == 0 || (j > 0 && !graph[i + 1][j - 1].is_ascii_digit()) {
                        num_count += 1;
                        mult *= span_num(&graph, i + 1, j);
                    }
                }
                if i + 1 < m
                    && j + 1 < n
                    && graph[i + 1][j + 1].is_ascii_digit()
                    && !graph[i + 1][j].is_ascii_digit()
                {
                    num_count += 1;
                    mult *= span_num(&graph, i + 1, j + 1);
                }
                if num_count == 2 {
                    println!("({},{})", i, j);
                    v.push(mult);
                }
            }
        }
    }
    let res: i32 = v.iter().sum();
    println!("Part2:{res}");
}

fn part1(input: &str) {
    let mut graph = input
        .lines()
        .map(|line| line.to_owned().as_bytes().to_vec())
        .collect::<Vec<_>>();
    let m = graph.len();
    let n = graph[0].len();
    let total_sum = sum_nums(&graph);
    for i in 0..m {
        for j in 0..n {
            if !graph[i][j].is_ascii_digit() && graph[i][j] != b'.' {
                // symbol
                delim_nums(&mut graph, i as i32, j as i32);
            }
        }
    }
    let remain_sum = sum_nums(&graph);
    println!("sum:{}", total_sum - remain_sum);
}

fn main() {
    let input = std::fs::read("src/day3/input.txt").unwrap();
    let input = std::str::from_utf8(&input).unwrap();
    part1(input);
    part2(input);
    // delim_nums(&mut graph, 8, 5);
    // let graph_owned = graph
    //     .iter()
    //     .map(|line| String::from_utf8(line.clone()))
    //     .filter_map(Result::ok)
    //     .collect::<Vec<_>>();
    // for line in graph_owned {
    //     println!("{line}");
    // }
}

fn span_num(matrix: &Vec<Vec<u8>>, x: usize, y: usize) -> i32 {
    // we discover a digit in matrix[x][y], we want to count the number that include the digit
    let m = matrix.len();
    let n = matrix[0].len();
    let start_x = x;
    let mut start_y = y;
    for i in (-1..y as i32 + 1).rev() {
        if i == -1 {
            start_y = 0;
            break;
        }
        if !matrix[start_x][i as usize].is_ascii_digit() {
            start_y = i as usize + 1;
            break;
        }
    }
    let mut ans = 0;
    // println!("+>({start_x},{start_y})");
    for j in (start_y as usize)..n {
        if !matrix[start_x][j].is_ascii_digit() {
            break;
        }
        ans = ans * 10 + ((matrix[start_x][j] - b'0') as i32);
    }
    ans
}

fn sum_nums(matrix: &Vec<Vec<u8>>) -> i32 {
    let mut nums = Vec::new();
    matrix
        .iter()
        .map(|line| {
            let mut num: Option<i32> = None;
            for ch in line {
                if ch.is_ascii_digit() {
                    if let Some(t) = num {
                        num = Some(t * 10 + (ch - b'0') as i32);
                    } else {
                        num = Some((ch - b'0') as i32);
                    }
                } else {
                    if let Some(t) = num {
                        num = None;
                        nums.push(t);
                    }
                }
            }
            if let Some(num) = num {
                nums.push(num);
            }
        })
        .collect::<Vec<_>>();
    nums.iter().sum::<i32>()
}

// matrix[x][y] is symbol
fn delim_nums(matrix: &mut Vec<Vec<u8>>, x: i32, y: i32) {
    let m: i32 = matrix.len() as i32;
    let n: i32 = matrix[0].len() as i32;
    let directions = vec![
        (0, -1),
        (-1, 0),
        (0, 1),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    for direct in directions {
        let x_move = x + direct.0;
        let y_move = y + direct.1;
        if x_move >= 0 && x_move < m && y_move >= 0 && y_move < n {
            let x_move = x_move as usize;
            let y_move = y_move as usize;
            let ch = matrix[x_move][y_move];
            if ch.is_ascii_digit() {
                for j in (0..y_move + 1).rev() {
                    if !matrix[x_move][j].is_ascii_digit() {
                        break;
                    }
                    matrix[x_move][j] = b'.';
                }
                for j in y_move + 1..n as usize {
                    if !matrix[x_move][j].is_ascii_digit() {
                        break;
                    }
                    matrix[x_move][j] = b'.';
                }
            }
        }
    }
}
