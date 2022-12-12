#![feature(iter_next_chunk)]
#![feature(slice_partition_dedup)]
#![feature(slice_flatten)]

mod common;
mod d7;
mod d8;

fn solve1() {
    let all = common::read_file_to_string("input1.txt".to_string());
    let line_iterator = all.lines();
    let mut carried_per_person = Vec::new();
    let mut current_calories = 0;
    for line in line_iterator {
        if line.len() == 0 {
            carried_per_person.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }
    carried_per_person.push(current_calories);
    carried_per_person.sort();

    let most_calories = carried_per_person.last().unwrap();
    println!("Most calories: {most_calories}");

    let last_index = carried_per_person.len() - 1;
    let second_index = carried_per_person.len() - 2;
    let third_index = carried_per_person.len() - 3;

    let last = carried_per_person.get(last_index).unwrap();
    let second = carried_per_person.get(second_index).unwrap();
    let third = carried_per_person.get(third_index).unwrap();
    let top_three = last + second + third;
    println!("TopThree {top_three}");
}

#[derive(PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Lose,
    Draw,
}

fn get_target_play(opponent_play: &Play, target_result: Result) -> Play {
    match opponent_play {
        Play::Rock => match target_result {
            Result::Win => return Play::Paper,
            Result::Draw => return Play::Rock,
            Result::Lose => return Play::Scissors,
        },
        Play::Paper => match target_result {
            Result::Win => return Play::Scissors,
            Result::Draw => return Play::Paper,
            Result::Lose => return Play::Rock,
        },
        Play::Scissors => match target_result {
            Result::Win => Play::Rock,
            Result::Draw => Play::Scissors,
            Result::Lose => Play::Paper,
        },
    }
}

fn char_to_play(c: char) -> Play {
    match c {
        'A' => return Play::Rock,
        'B' => return Play::Paper,
        'C' => return Play::Scissors,
        'X' => return Play::Rock,
        'Y' => return Play::Paper,
        'Z' => return Play::Scissors,
        _ => todo!(),
    }
}

fn char_to_result(c: char) -> Result {
    match c {
        'X' => return Result::Lose,
        'Y' => return Result::Draw,
        'Z' => return Result::Win,
        _ => todo!(),
    }
}

fn get_play_result(enemy: Play, us: &Play) -> Result {
    match enemy {
        Play::Rock => match us {
            Play::Rock => return Result::Draw,
            Play::Paper => return Result::Win,
            Play::Scissors => return Result::Lose,
        },
        Play::Paper => match us {
            Play::Rock => return Result::Lose,
            Play::Paper => return Result::Draw,
            Play::Scissors => return Result::Win,
        },
        Play::Scissors => match us {
            Play::Rock => return Result::Win,
            Play::Paper => return Result::Lose,
            Play::Scissors => return Result::Draw,
        },
    }
}

fn get_rps_score(line: &str, new_method: bool) -> i32 {
    assert_eq!(line.len(), 3);
    let opponent_char = line.chars().nth(0).unwrap();
    let our_char = line.chars().nth(2).unwrap();

    let opponent_play = char_to_play(opponent_char);
    let target_result = char_to_result(our_char);

    let our_play = if new_method {
        get_target_play(&opponent_play, target_result)
    } else {
        char_to_play(our_char)
    };

    let mut score = 0;
    let result = get_play_result(opponent_play, &our_play);
    match result {
        Result::Draw => score += 3,
        Result::Win => score += 6,
        Result::Lose => score += 0,
    }

    match our_play {
        Play::Rock => score += 1,
        Play::Paper => score += 2,
        Play::Scissors => score += 3,
    }

    return score;
}

fn solve2() {
    let all = common::read_file_to_string("input2.txt".to_string());
    let line_iterator = all.lines();

    let mut old_score = 0;
    let mut new_score = 0;
    for line in line_iterator {
        old_score += get_rps_score(line, false);
        new_score += get_rps_score(line, true);
    }

    println!("currentScore: {old_score}, new score: {new_score}");
}

fn split_in_half(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn get_match(a: &str, b: &str) -> char {
    for a_char in a.chars() {
        for b_char in b.chars() {
            if a_char == b_char {
                return a_char;
            }
        }
    }
    panic!("We must find a match");
}

fn get_match_trio(a: &str, b: &str, c: &str) -> char {
    for a_char in a.chars() {
        for b_char in b.chars() {
            if a_char == b_char {
                for c_char in c.chars() {
                    if c_char == b_char {
                        return c_char;
                    }
                }
            }
        }
    }
    panic!("We must find a match");
}

fn get_letter_score(letter: char) -> u32 {
    if letter.is_lowercase() {
        return letter as u32 - 'a' as u32 + 1;
    } else {
        return letter as u32 - 'A' as u32 + 27;
    }
}

fn process_trio(chunk: [&str; 3]) -> u32 {
    let a = chunk[0];
    let b = chunk[1];
    let c = chunk[2];

    let matching = get_match_trio(a, b, c);

    return get_letter_score(matching);
}

fn solve3() {
    let all = common::read_file_to_string("input3.txt".to_string());

    let line_iterator = all.lines();

    let mut total_priority = 0;
    for line in line_iterator {
        let (first, second) = split_in_half(line);
        let matching_letter = get_match(first, second);
        total_priority += get_letter_score(matching_letter);
    }

    let mut badge_prio = 0;
    let mut second_line_iterator = all.lines();
    loop {
        badge_prio += match second_line_iterator.next_chunk::<3>() {
            Ok(val) => process_trio(val),
            Err(_) => break,
        }
    }
    println!("total prio {total_priority}, badge prio {badge_prio}")
}

struct Range(i32, i32);

impl Range {
    fn is_subset(&self, other: &Range) -> bool {
        let self_is_subset = self.0 >= other.0 && self.1 <= other.1;
        let other_is_subset = other.0 >= self.0 && other.1 <= self.1;
        return self_is_subset || other_is_subset;
    }
    fn overlaps(&self, other: &Range) -> bool {
        let beginning_inside = self.0 >= other.0 && self.0 <= other.1;
        let end_inside = self.1 >= other.0 && self.1 <= other.1;
        return beginning_inside || end_inside || self.is_subset(other);
    }
}

fn parse_range(dash_string: &str) -> Range {
    let mut numbers = dash_string.split('-');
    let first = numbers.next().unwrap();
    let second = numbers.next().unwrap();
    return Range(
        first.parse::<i32>().unwrap(),
        second.parse::<i32>().unwrap(),
    );
}

fn line_to_ranges(line: &str) -> (Range, Range) {
    let mut ranges = line.split(",");
    let first_range = ranges.next().unwrap();
    let second_range = ranges.next().unwrap();
    return (parse_range(first_range), parse_range(second_range));
}

fn solve4() {
    let all = common::read_file_to_string("input4.txt".to_string());
    let lines = all.lines();
    let mut redundant_count = 0;
    let mut overlap_count = 0;
    for line in lines {
        let ranges = line_to_ranges(line);
        if ranges.0.is_subset(&ranges.1) {
            redundant_count += 1;
        }
        if ranges.0.overlaps(&ranges.1) {
            overlap_count += 1;
        }
    }

    println!("Redundants: {redundant_count}, overlaps: {overlap_count}");
}

struct CraneInstruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl CraneInstruction {
    fn from_str(str: &str) -> CraneInstruction {
        let parts: Vec<&str> = str.split(' ').collect();
        let amount_str = parts[1];
        let from_str = parts[3];
        let to_str = parts[5];
        CraneInstruction {
            amount: amount_str.parse().unwrap(),
            from: from_str.parse().unwrap(),
            to: to_str.parse().unwrap(),
        }
    }
}

struct CraneSystem {
    stacks: Vec<Vec<char>>,
}

impl CraneSystem {
    fn new() -> CraneSystem {
        CraneSystem { stacks: Vec::new() }
    }

    fn handle_possible_crate(&mut self, crane_crate: &str, stack_id: usize) {
        let box_type = crane_crate.chars().nth(1).unwrap();
        if !box_type.is_alphabetic() {
            return;
        }

        if self.stacks.len() <= stack_id {
            self.stacks.resize(stack_id + 1, Vec::new());
        }

        self.stacks.get_mut(stack_id).unwrap().push(box_type);
    }

    fn handle_instruction(&mut self, instruction: CraneInstruction, new_mover: bool) {
        if !new_mover {
            for _ in 0..instruction.amount {
                let crate_type;
                {
                    let src = self.stacks.get_mut(instruction.from - 1).unwrap();
                    crate_type = src.pop().unwrap();
                }
                let dst = self.stacks.get_mut(instruction.to - 1).unwrap();
                dst.push(crate_type);
            }
        } else {
            let mut crates_in_air: Vec<char>;
            {
                let src = self.stacks.get_mut(instruction.from - 1).unwrap();
                let removed_range = (src.len() - instruction.amount)..(src.len());
                crates_in_air = src.drain(removed_range).collect();
            }
            let dst = self.stacks.get_mut(instruction.to - 1).unwrap();
            dst.append(&mut crates_in_air);
        }
    }

    fn print_top(&self) {
        for stack in &self.stacks {
            let last_char = stack.last().unwrap();
            print!("{last_char}");
        }
    }
    fn flip(&mut self) {
        for stack in self.stacks.iter_mut() {
            stack.reverse();
        }
    }
}

fn solve5(new_mover: bool) {
    let all = common::read_file_to_string("input5.txt".to_string());
    let lines = all.lines();
    let mut stacks_over: bool = false;

    let mut crane = CraneSystem::new();
    for line in lines {
        if line.is_empty() {
            stacks_over = true;
            crane.flip();
            continue;
        }

        if !stacks_over {
            let mut current_stack = 0;
            let mut chars = line.chars();
            loop {
                let chunk = chars.next_chunk::<4>();
                match chunk {
                    Ok(maybe_crate) => {
                        let maybe_crate_string = String::from_iter(maybe_crate);
                        crane.handle_possible_crate(maybe_crate_string.as_str(), current_stack);
                    }
                    Err(maybe_crate) => {
                        let maybe_crate_string = String::from_iter(maybe_crate);
                        crane.handle_possible_crate(maybe_crate_string.as_str(), current_stack);
                        break;
                    }
                }
                current_stack += 1;
            }
        } else {
            let instruction = CraneInstruction::from_str(line);
            crane.handle_instruction(instruction, new_mover);
        }
    }

    println!("Top of stacks, {}", crane.stacks.len());
    crane.print_top();
    println!("");
}

fn is_start_of_identifier(chars: Vec<char>, len: usize) -> bool {
    let mut sorted_chars: Vec<char> = chars;
    sorted_chars.sort();
    let (uniques, _duplications) = sorted_chars.partition_dedup();
    uniques.len() == len
}

fn solve6(header_len: usize) {
    let datastream = common::read_file_to_string("input6.txt".to_string());
    for i in header_len..datastream.len() {
        let start = i - header_len;
        let maybe_start: Vec<char> = datastream.chars().skip(start).take(header_len).collect();

        if is_start_of_identifier(maybe_start, header_len) {
            println!("found at {i}");
            return;
        }
    }
}

fn main() {
    solve1();
    solve2();
    solve3();
    solve4();
    solve5(false);
    solve5(true);
    solve6(4);
    solve6(14);
    d7::solve();
    d8::solve();
}
