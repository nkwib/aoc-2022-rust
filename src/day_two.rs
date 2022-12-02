use std::collections::HashMap;

pub fn solve(input: String) {
    let mapped_values: HashMap<String, Vec<i32>> = HashMap::from([
        (String::from("A X"), vec![4, 3]),
        (String::from("A Y"), vec![8, 4]),
        (String::from("A Z"), vec![3, 8]),
        (String::from("B X"), vec![1, 1]),
        (String::from("B Y"), vec![5, 5]),
        (String::from("B Z"), vec![9, 9]),
        (String::from("C X"), vec![7, 2]),
        (String::from("C Y"), vec![2, 6]),
        (String::from("C Z"), vec![6, 7]),
    ]);

    let games = input.split("\n");
//     replace game with MAPPED_VALUES[game]
    let mut points: Vec<Vec<i32>> = Vec::new();

    for game in games {
        points.push(mapped_values.get(game).unwrap().to_vec());
    }
    let mut first_score = 0;
    let mut second_score = 0;
    for point in points {
        first_score += point[0];
        second_score += point[1];
    }
    println!("Playing your interpretation of Rock, Paper, Scissors will give you: {}", first_score);
    println!("Playing as intended by the Elf at Rock, Paper, Scissors will give you: {}", second_score);
}