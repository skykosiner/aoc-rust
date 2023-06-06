use std::fs;

#[derive(Debug)]
struct Elf {
    calories: i32,
}

fn parse_line(items: std::str::Split<'_, &str>) -> Vec<Elf>  {
    let mut elfs: Vec<Elf> = Vec::new();
    let mut items_to_clear: Vec<i32> = Vec::new();

    for item in items {
        if item == "" {
            let mut total: i32 = 0;
            for cal in &items_to_clear {
                total += cal;
            };

            elfs.push(Elf { calories: total });
            items_to_clear.clear();
        } else {
            let cal = item.to_string().parse::<i32>().unwrap();
            items_to_clear.push(cal);
        };
    };

    return elfs;
}

fn part_one(elfs: &Vec<Elf>) {
    let mut largest_int: i32 = 0;

    for elf in elfs {
        if elf.calories > largest_int {
            largest_int = elf.calories;
        };
    };

    println!("{}", largest_int);
}

fn part_two(elfs: &Vec<Elf>) {
    let mut output: i32 = 0;
    let mut numbers: Vec<i32> = Vec::new();

    for elf in elfs {
        numbers.push(elf.calories);
        numbers.sort();
    };

    let last_three = numbers.iter().rev().take(3);
    for last in last_three {
        output += last;
    };

    println!("{}", output);
}

fn main() {
    let items = fs::read_to_string("./src/day1.input").expect("file not found");
    let elfs = parse_line(items.split("\n"));

    part_one(&elfs);
    part_two(&elfs);
}
