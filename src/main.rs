use std::env;
mod days;

fn main() {
    let test: bool = false;
    let args: Vec<String> = env::args().collect();
    let day: i32 = args[args.len() - 1].parse().unwrap();
    // if day is imbertween 1 and 25, run the day
    if day > 0 && day < 26 {
        days::run(day, test);
    } else {
        println!("Please enter a day between 1 and 25");
    }
}