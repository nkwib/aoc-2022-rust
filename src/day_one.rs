pub fn solve_first(input: String) {
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
    println!("The elf with the most calories is: {}", richest_elf);
}

pub fn solve_second(input: String) {
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
    println!("The sum of the elves with the most calories is: {}", total);
}

pub fn solve(input: String) {
    solve_first(input.clone());
    solve_second(input.clone());
}