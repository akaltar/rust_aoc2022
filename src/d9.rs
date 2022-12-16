use crate::common;

#[derive(Debug)]
struct Point(i32, i32);

fn adjust_tail(head: &mut Point, tail: &mut Point, move_vec: Point, all_tail_pos: &mut Vec<Point>) {
    let mut abs_x = move_vec.0.abs();
    let mut abs_y = move_vec.1.abs();

    if (abs_x == 1 && abs_y > 1) || (abs_x > 1 && abs_y == 1) {
        // Move diagonally

        if move_vec.0 > 0 {
            tail.0 -= 1;
        } else {
            tail.0 += 1;
        }

        if move_vec.1 > 0 {
            tail.1 -= 1;
        } else {
            tail.1 += 1;
        }
        abs_x -= 1;
        abs_y -= 1;
        all_tail_pos.push(Point(tail.0, tail.1));
    }

    while abs_x > 1 {
        if move_vec.0 > 0 {
            tail.0 -= 1;
        } else {
            tail.0 += 1;
        }
        abs_x -= 1;
        all_tail_pos.push(Point(tail.0, tail.1));
    }

    while abs_y > 1 {
        if move_vec.1 > 0 {
            tail.1 -= 1;
        } else {
            tail.1 += 1;
        }
        abs_y -= 1;
        all_tail_pos.push(Point(tail.0, tail.1));
    }
}

pub fn solve() {
    println!("Day 9:");

    let all = common::read_file_to_string("input9.txt".to_string());
    let lines = all.lines();

    let mut all_tail_pos = Vec::new();
    all_tail_pos.push(Point(0, 0));

    let mut tail_pos = Point(0, 0);
    let mut head_pos = Point(0, 0);
    let mut total_amount = 0;
    for line in lines {
        let (direction, amount_string) = line.split_once(' ').unwrap();
        let amount = amount_string.to_string().parse::<i32>().unwrap();
        total_amount += amount;
        let move_vec = match direction {
            "U" => Point(0, amount),
            "D" => Point(0, -amount),
            "R" => Point(amount, 0),
            "L" => Point(-amount, 0),
            _ => {
                panic!();
            }
        };

        adjust_tail(&mut head_pos, &mut tail_pos, move_vec, &mut all_tail_pos);
        println!(
            "directi:{direction}, amount:{amount_string} head: ({head_pos:?}) tail:({tail_pos:?}"
        );
    }

    println!("Visited all: {all_tail_pos:?}");
    all_tail_pos.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.partial_cmp(&b.1).unwrap()
        } else {
            a.0.partial_cmp(&b.0).unwrap()
        }
    });
    all_tail_pos.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);

    // too high: 1991045
    // too low: 6632
    println!("total moves: {total_amount}");
    println!("all visited locations: {}", all_tail_pos.len());
}
