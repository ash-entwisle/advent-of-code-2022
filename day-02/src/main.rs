use std::fs;
mod tasks;

fn main() {
    let data: String = fs::read_to_string("./src/data.txt").unwrap();
    println!("Score 1: {}", tasks::one::run(&data));
    println!("Score 2: {}", tasks::two::run(&data));
}


