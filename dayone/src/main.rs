use std::fs;

#[derive(Debug)]
struct Elf {
    calories: i32,
}

fn parse_lines(items: std::str::Split<'_, &str>) -> Vec<Elf>  {
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

fn solve(elfs: &Vec<Elf>, nuber_to_take: usize) {
    let mut numbers: Vec<i32> = Vec::new();
    for elf in elfs {
        numbers.push(elf.calories);
        numbers.sort();
    }

    println!("{}", numbers.iter().rev().take(nuber_to_take).sum::<i32>());
}

fn main() {
    let items = fs::read_to_string("./src/day1.input").expect("file not found");
    let elfs = parse_lines(items.split("\n"));

    // Part 1
    solve(&elfs, 1);
    // Part 2
    solve(&elfs, 3);
}
