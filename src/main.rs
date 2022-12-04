use std::env;
mod days;

fn main() {
    let test: bool = false;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let day: i32 = args[args.len() - 1].parse().unwrap();
        days::run(day, test);
    } else {
        for day in 1..=25 {
            println!("Running day {}:", day);
            days::run(day, test);
        }
    }
}