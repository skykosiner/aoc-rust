use std::{fs};

fn get_int_value(ch: char) -> i32 {
    let ascii = ch as i32;
    // Upper case
    if ascii <= 65 || ascii <= 90 {
        return ascii - 38;
    } else {
        // Lower case
        return ascii - 96;
    }
}

fn part_one(lines: std::str::Split<'_, &str> ) {
    let mut chars: Vec<i32> = Vec::new();
    for line in lines {
        if line == "" {
            return
        }

        let split_len = line.len() / 2;
        let (str1, str2) = line.split_at(split_len);
        let mut seen: bool = false;
        for char in str1.chars() {
            for char2 in str2.chars() {
                if char == char2 {
                    if !seen {
                        chars.push(get_int_value(char));
                        seen = true;
                        break
                    }
                }
            }
        }
        println!("{}", chars.iter().sum::<i32>());
    }
}

fn part_two(lines: std::str::Split<'_, &str>,) {
    for line in lines {
    }
}

fn main() {
    let lines = fs::read_to_string("./src/day3.test").expect("Erorr reading file");
    part_one(lines.split("\n"));
    part_two(lines.split("\n"));
}
