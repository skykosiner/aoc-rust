fn part_one(lines: Vec<&str>) {
    let mut total: i32 = 0;
    for line in lines {
        let mut part_one: i32 = 0;
        let mut part_two: i32 = 0;
        let mut part_three: i32 = 0;
        let mut part_four: i32 = 0;

        part_one = line.split(",").collect::<Vec<&str>>()[0].split("-").collect::<Vec<&str>>()[0].to_string().parse::<i32>().unwrap();
        part_two = line.split(",").collect::<Vec<&str>>()[0].split("-").collect::<Vec<&str>>()[1].to_string().parse::<i32>().unwrap();
        part_three = line.split(",").collect::<Vec<&str>>()[1].split("-").collect::<Vec<&str>>()[0].to_string().parse::<i32>().unwrap();
        part_four = line.split(",").collect::<Vec<&str>>()[1].split("-").collect::<Vec<&str>>()[1].to_string().parse::<i32>().unwrap();

        println!("{} {} {} {}", part_one, part_two, part_three, part_four);

        if part_one >= part_three && part_two <= part_four {
            total += 1;
        } else if part_three >= part_one && part_four <= part_two {
            total += 1;
        }
    }

    println!("{}", total);
}

fn main() {
    let file = include_str!("./day4.input");
    let lines: Vec<_> = file
        .split("\n")
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect();

    part_one(lines);
}
