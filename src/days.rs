use std::fs;
mod day01;
mod day02;
mod day03;

pub fn run(day: i32) {
    let data = fs::read_to_string(format!("src/days/day{:02}/data/data.txt", day)).expect("Unable to read file");
    match day {
        1 => day01::run(data),
        2 => day02::run(data),
        3 => day03::run(data),
        i32::MIN..=0_i32 | 2_i32..=i32::MAX => println!("Day {} is not implemented yet", day),
    };
}