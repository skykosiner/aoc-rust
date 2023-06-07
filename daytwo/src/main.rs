use std::{fs, collections::HashMap};

fn parse_lines(items: std::str::Split<'_, char>) {
    let mut total_one: i32 = 0;
    let mut total_two: i32 = 0;

    let mut points_part_one: HashMap<&str, i32> = HashMap::new();
    points_part_one.insert("A X", 4);
    points_part_one.insert("A Y", 8);
    points_part_one.insert("A Z", 3);
    points_part_one.insert("B X", 1);
    points_part_one.insert("B Y", 5);
    points_part_one.insert("B Z", 9);
    points_part_one.insert("C X", 7);
    points_part_one.insert("C Y", 2);
    points_part_one.insert("C Z", 6);

    let mut points_part_two: HashMap<&str, i32> = HashMap::new();
    points_part_two.insert("A X", 3);
    points_part_two.insert("A Y", 4);
    points_part_two.insert("A Z", 8);
    points_part_two.insert("B X", 1);
    points_part_two.insert("B Y", 5);
    points_part_two.insert("B Z", 9);
    points_part_two.insert("C X", 2);
    points_part_two.insert("C Y", 6);
    points_part_two.insert("C Z", 7);

    for item in items {
        total_one += points_part_one.get(item).unwrap_or(&0);
        total_two += points_part_two.get(item).unwrap_or(&0);
    };

    println!("{}", total_one);
    println!("{}", total_two);
}

fn main() {
    let data = fs::read_to_string("./src/day2.input").expect("Error reading file");
    parse_lines(data.split('\n'));
}
