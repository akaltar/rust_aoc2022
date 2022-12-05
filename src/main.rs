#![feature(iter_next_chunk)]

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

fn read_file_to_string(file_name: String) -> String {
    let path = Path::new(&file_name);
    let display = path.display();
  
    let mut file = match File::open(&path) {
      Err(why) => panic!("Couldn't open {}: {}", display, why),
      Ok(file) => file,
    };
    let mut string = String::new();
  
    match file.read_to_string(&mut string) {
      Err(why) => panic!("Couldn't read {}:{}", display, why),
      Ok(_) => string,
    }
}

fn solve1() {
    let all = read_file_to_string("input1.txt".to_string());
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
    Draw
}

fn get_target_play(opponent_play:&Play, target_result:Result) -> Play {
    match opponent_play {
        Play::Rock => {
            match target_result {
                Result::Win => return Play::Paper,
                Result::Draw => return Play::Rock,
                Result::Lose => return Play::Scissors,
            }
        },
        Play::Paper => {
            match target_result {
                Result::Win => return Play::Scissors,
                Result::Draw => return Play::Paper,
                Result::Lose => return Play::Rock,
            }
        },
        Play::Scissors => {
            match target_result {
                Result::Win => Play::Rock,
                Result::Draw => Play::Scissors,
                Result::Lose => Play::Paper,
            }
        }
    }
}

fn char_to_play(c:char) -> Play {
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

fn char_to_result(c:char) -> Result {
    match c {
        'X' => return Result::Lose,
        'Y' => return Result::Draw,
        'Z' => return Result::Win,
        _ => todo!(),
    }
}

fn get_play_result(enemy: Play, us: &Play) -> Result {
    match enemy {
        Play::Rock => {
            match us {
                Play::Rock => return Result::Draw,
                Play::Paper => return Result::Win,
                Play::Scissors => return Result::Lose,
            }
        },
        Play::Paper => {
            match us {
                Play::Rock => return Result::Lose,
                Play::Paper => return Result::Draw,
                Play::Scissors => return Result::Win,
            }
        },
        Play::Scissors => {
            match us {
                Play::Rock => return Result::Win,
                Play::Paper => return Result::Lose,
                Play::Scissors => return Result::Draw
            }
        }
    }
}

fn get_rps_score(line:&str, new_method: bool) -> i32 {
    assert_eq!(line.len(), 3);
    let opponent_char = line.chars().nth(0).unwrap();
    let our_char = line.chars().nth(2).unwrap();

    let opponent_play = char_to_play(opponent_char);
    let target_result = char_to_result(our_char);
    
    let our_play =  if new_method {get_target_play(&opponent_play, target_result) } else {char_to_play(our_char)};

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
    let all = read_file_to_string("input2.txt".to_string());
    let line_iterator = all.lines();

    let mut old_score = 0;
    let mut new_score = 0;
    for line in line_iterator {
        old_score += get_rps_score(line, false);
        new_score += get_rps_score(line, true);
    }

    println!("currentScore: {old_score}, new score: {new_score}");
}

fn split_in_half(line:&str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn get_match(a:&str, b:&str) -> char {
    for a_char in a.chars() {
        for b_char in b.chars() {
            if a_char == b_char {return a_char}
        }
    }
    panic!("We must find a match");
}

fn get_match_trio(a:&str, b:&str, c:&str) -> char {
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

fn get_letter_score(letter:char) -> u32 {
    if letter.is_lowercase()  {
        return letter as u32 - 'a' as u32 + 1;
    } else {
        return letter as u32 - 'A' as u32 + 27;
    }
}

fn process_trio(chunk: [&str;3]) -> u32 {
    let a = chunk[0];
    let b = chunk[1];
    let c = chunk[2];

    let matching = get_match_trio(a,b,c);

    return get_letter_score(matching);
}

fn solve3() {
    let all = read_file_to_string("input3.txt".to_string());

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
        badge_prio += match second_line_iterator.next_chunk::<3>()
        {
            Ok(val) => {
                process_trio(val)
            },
            Err(_) => break
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
    return Range(first.parse::<i32>().unwrap(), second.parse::<i32>().unwrap());
}

fn line_to_ranges(line: &str) -> (Range, Range) {
    let mut ranges = line.split(",");
    let first_range = ranges.next().unwrap();
    let second_range = ranges.next().unwrap();
    return (parse_range(first_range), parse_range(second_range));
}

fn solve4() {
    let all = read_file_to_string("input4.txt".to_string());
    let lines = all.lines();
    let mut redundant_count = 0;
    let mut overlap_count = 0;
    for line in lines {
        let ranges = line_to_ranges(line);
        if ranges.0.is_subset(&ranges.1) { redundant_count += 1; }
        if ranges.0.overlaps(&ranges.1) { overlap_count += 1;}
    }

    println!("Redundants: {redundant_count}, overlaps: {overlap_count}");

}

fn main() {
    solve1();
    solve2();
    solve3();
    solve4();
}