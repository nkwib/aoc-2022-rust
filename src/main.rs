use std::fs::File;
use std::io::Read;

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod utils;

fn read_file(name: &str) -> String {
    let mut file = File::open(name).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

fn main() {
    day_one::solve(read_file("src/input_one.txt"));
    day_two::solve(read_file("src/input_two.txt"));
    day_three::solve(read_file("src/input_three.txt"));
    day_four::solve(read_file("src/input_four.txt"));
}