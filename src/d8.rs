use crate::common;

pub fn solve() {
    // more than 526
    // more than 1033
    // more than 1381
    println!("Day 8:");

    let all = common::read_file_to_string("input8.txt".to_string());
    let lines = all.lines();
    const SIZE: usize = 99;
    let mut vis_matrix: [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];
    let mut height_matrix: [[i64; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    for (line_index, line) in lines.enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            let height = char as i64 - '0' as i64;
            height_matrix[line_index][char_index] = height;
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
            if height_matrix[x][y] > highest_x {
                highest_x = height_matrix[x][y];
                vis_matrix[x][y] = true;
            }

            // bottom up
            if height_matrix[x][y_r] > highest_x_r {
                highest_x_r = height_matrix[x][y_r];
                vis_matrix[x][y_r] = true;
            }

            // left to right
            if height_matrix[y][x] > highest_y {
                highest_y = height_matrix[y][x];
                vis_matrix[y][x] = true;
            }

            // right to left
            if height_matrix[y_r][x] > highest_y_r {
                highest_y_r = height_matrix[y_r][x];
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
}
