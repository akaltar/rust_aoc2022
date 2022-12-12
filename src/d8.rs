use crate::common;

const SIZE: usize = 99;
type Forest = [[i64; SIZE]; SIZE];

// too low: 611520
fn get_tree_scenic_score(x: usize, y: usize, forest: Forest) -> i64 {
    let to_left = (0..x).rev();
    let to_right = (x + 1)..SIZE;
    let to_top = (0..y).rev();
    let to_bottom = (y + 1)..SIZE;

    let our_height = forest[x][y];

    let mut left_score = 0;
    let mut right_score = 0;
    let mut top_score = 0;
    let mut bottom_score = 0;
    for look_x in to_left {
        left_score += 1;
        if forest[look_x][y] >= our_height {
            break;
        }
    }

    for look_x in to_right {
        right_score += 1;
        if forest[look_x][y] >= our_height {
            break;
        }
    }

    for look_y in to_top {
        top_score += 1;
        if forest[x][look_y] >= our_height {
            break;
        }
    }

    for look_y in to_bottom {
        bottom_score += 1;
        if forest[x][look_y] >= our_height {
            break;
        }
    }

    //println!("lrtp: {left_score}, {right_score} {top_score} {bottom_score} ");

    left_score * right_score * top_score * bottom_score
}

pub fn solve() {
    // more than 526
    // more than 1033
    // more than 1381
    println!("Day 8:");

    let all = common::read_file_to_string("input8.txt".to_string());
    let lines = all.lines();
    let mut vis_matrix: [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];
    let mut forest: Forest = [[0; SIZE]; SIZE];

    for (line_index, line) in lines.enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            let height = char as i64 - '0' as i64;
            forest[line_index][char_index] = height;
        }
    }

    for x in 0..SIZE {
        let mut highest_x: i64 = -1;
        let mut highest_y: i64 = -1;
        let mut highest_x_r: i64 = -1;
        let mut highest_y_r: i64 = -1;

        for y in 0..SIZE {
            let y_r = SIZE - y - 1;

            // top down
            if forest[x][y] > highest_x {
                highest_x = forest[x][y];
                vis_matrix[x][y] = true;
            }

            // bottom up
            if forest[x][y_r] > highest_x_r {
                highest_x_r = forest[x][y_r];
                vis_matrix[x][y_r] = true;
            }

            // left to right
            if forest[y][x] > highest_y {
                highest_y = forest[y][x];
                vis_matrix[y][x] = true;
            }

            // right to left
            if forest[y_r][x] > highest_y_r {
                highest_y_r = forest[y_r][x];
                vis_matrix[y_r][x] = true;
            }
        }
    }

    let mut all_visible = 0;
    for visible in vis_matrix.flatten() {
        if *visible {
            all_visible += 1;
        }
    }
    println!("Number of visible trees: {all_visible}");

    let mut all_scores = Vec::new();
    for x in 0..SIZE {
        for y in 0..SIZE {
            all_scores.push(get_tree_scenic_score(x, y, forest))
        }
    }
    all_scores.sort();

    let least_scenic = all_scores.first().unwrap();
    let highest_scenic = all_scores.last().unwrap();

    println!(
        "Highest scenic:{highest_scenic}, {least_scenic}, num: {}",
        all_scores.len()
    );
}
