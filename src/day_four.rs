use crate::utils;

fn solve_first(input: Vec<[[i32; 2]; 2]>) {
    let mut inclusions = 0;
    for pair in input {
        if utils::one_contains_the_other(pair[0], pair[1]) {
            inclusions += 1;
        }
    }
    println!("The pairs that includes the other: {}", inclusions);
}

fn solve_second(input: Vec<[[i32; 2]; 2]>) {
    let mut overlaps = 0;
    for pair in input {
        if utils::one_overlaps_the_other(pair[0], pair[1]) {
            overlaps += 1;
        }
    }
    println!("The pairs that overlaps the other: {}", overlaps);
}


pub fn solve(input: String) {
    let lines = input.lines();
    let mut res: Vec<[[i32; 2]; 2]> = Vec::new();
    for line in lines {
        let first = line.split(",").nth(0).unwrap();
        let second = line.split(",").nth(1).unwrap();
        let pair_1 = first.split("-").collect::<Vec<&str>>();
        let pair_2 = second.split("-").collect::<Vec<&str>>();
        res.push([
            [
                pair_1[0].parse::<i32>().unwrap(),
                pair_1[1].parse::<i32>().unwrap()
            ], [
                pair_2[0].parse::<i32>().unwrap(),
                pair_2[1].parse::<i32>().unwrap()
            ]
        ]);
    }
    solve_first(res.clone());
    solve_second(res.clone());
}