use std::{fs};

fn get_int_value(ch: char) -> i32 {
    let ascii_char = ch as i32;

    // Upper case
    if ascii_char <= 65 || ascii_char <= 90 {
        return ascii_char - 38;
    }

    // Lower case
    ascii_char - 96
}

fn part_one(lines: std::str::Split<'_, char> ) {
    let mut chars: Vec<i32> = Vec::new();
    for line in lines {
        if line.is_empty() {
            return
        }

        let split_len = line.len() / 2;
        let (str1, str2) = line.split_at(split_len);
        let mut seen: bool = false;
        for char in str1.chars() {
            for char2 in str2.chars() {
                if char == char2 && !seen {
                    chars.push(get_int_value(char));
                    seen = true;
                    break
                }
            }
        }
        println!("{}", chars.iter().sum::<i32>());
    }
}

fn part_two(lines: std::str::Split<'_, char>,) {
    let mut items: Vec<&str> = Vec::new();
    let mut chars: Vec<i32> = Vec::new();

    for line in lines {
        items.push(line);
    }

    for chunk in items.chunks_exact(3) {
        let mut seen: bool = false;
        let (chunk1, chunk2, chunk3): (&str, &str, &str) = (chunk[0], chunk[1], chunk[2]);
        for char1 in chunk1.chars() {
            for char2 in chunk2.chars() {
                for char3 in chunk3.chars() {
                    if char1 == char2 && char2 == char3 && !seen {
                        chars.push(get_int_value(char1));
                        seen = true;
                        break
                    }
                }
            }
        }
        println!("{}", chars.iter().sum::<i32>());
    }
}


fn main() {
    let lines = fs::read_to_string("./src/day3.input").expect("Erorr reading file");
    part_one(lines.split('\n'));
    part_two(lines.split('\n'));
}
