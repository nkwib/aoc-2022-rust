use std::fs::File;
use std::io::Read;

mod day_one;

fn read_file(name: &str) -> String {
    let mut file = File::open(name).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

fn main() {
    let input = read_file("src/input_one.txt");
    let input_two = input.clone();
    let solution_one = day_one::solve_first(input);
    let solution_two = day_one::solve_second(input_two);
    println!("The elf with the most calories is: {}", solution_one);
    println!("The sum of the elves with the most calories is: {}", solution_two);
}