use std::fs;

#[derive(Debug)]
struct Elf {
    calories: i32,
}

fn parse_lines(items: std::str::Split<'_, char>) -> Vec<Elf>  {
    let mut elfs: Vec<Elf> = Vec::new();
    let mut items_to_clear: Vec<i32> = Vec::new();

    for item in items {
        if item.is_empty() {
            let total: i32 = items_to_clear.iter().sum();
            elfs.push(Elf { calories: total });
            items_to_clear.clear();
        } else {
            let cal = item.to_string().parse::<i32>().unwrap();
            items_to_clear.push(cal);
        };
    };

    elfs
}

fn solve(elfs: &[Elf], nuber_to_take: usize) {
    let mut numbers: Vec<_> = elfs
        .iter()
        .map(|elf| elf.calories)
        .collect();

    numbers.sort();

    println!("{}", numbers.iter().rev().take(nuber_to_take).sum::<i32>());
}

fn main() {
    let items = fs::read_to_string("./src/day1.input").expect("file not found");
    let elfs = parse_lines(items.split('\n'));

    // Part 1
    solve(&elfs, 1);
    // Part 2
    solve(&elfs, 3);
}
