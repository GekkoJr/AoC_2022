use std::fs;

fn main() {
    let file: Vec<str> = fs::read_to_string("input.txt").unwrap().lines().collect();

    for line in file {
        println!("{}", line);
    }
}