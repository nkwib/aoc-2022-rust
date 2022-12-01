pub fn solve_first(input: String) -> i32 {
    // split by double newline
    let elves = input.split("\n\n");
    let mut sums: Vec<i32> = Vec::new();
    for elf in elves {
        // split by newline
        let calories = elf.split("\n");
        let mut total = 0;
        for food in calories {
            total += food.parse::<i32>().unwrap();
        }
        sums.push(total);
    }
    let richest_elf = sums.iter().max().unwrap();
    *richest_elf
}

pub fn solve_second(input: String) -> i32 {
    let elves = input.split("\n\n");
    let mut sums: Vec<i32> = Vec::new();
    for elf in elves {
        let calories = elf.split("\n");
        let mut total = 0;
        for food in calories {
            total += food.parse::<i32>().unwrap();
        }
        sums.push(total);
    }
    sums.sort();
    sums.reverse();
    let mut total = 0;
    for i in 0..3 {
        total += sums[i];
    }
    total
}