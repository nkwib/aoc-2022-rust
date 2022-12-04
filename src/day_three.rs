use crate::utils;

fn solve_first(input: String) {
    let mut priorities: Vec<i32> = Vec::new();
    let lines = input.lines();
    for line in lines {
        let (first, second) = utils::split_string_in_half(line.to_string());
        let common_char: char = utils::get_common_chars(vec![first, second]);
        priorities.push(utils::get_priority(common_char));
    }
    println!("The sum of priorities of the items type is: {}", priorities.iter().sum::<i32>());
}

fn solve_second(input: String) {
//     sliding window of 3
    let mut priorities: Vec<i32> = Vec::new();
    let lines = input.lines();
    let length = lines.clone().count();
    let mut i = 2;
    while i < length {
        let line1 = lines.clone().nth(i).unwrap();
        let line2 = lines.clone().nth(i - 1).unwrap();
        let line3 = lines.clone().nth(i - 2).unwrap();
        let common_char: char = utils::get_common_chars(vec![line1.to_string(), line2.to_string(), line3.to_string()]);
        priorities.push(utils::get_priority(common_char));
        i += 3;
    }
    println!("The sum of priorities for the badges is: {}", priorities.iter().sum::<i32>());
}


pub fn solve(input: String) {

    solve_first(input.clone());
    solve_second(input.clone());
}