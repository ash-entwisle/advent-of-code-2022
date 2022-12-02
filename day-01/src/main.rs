use std::fs;
mod tasks;

fn main() {
    let data: String = fs::read_to_string("./src/data.txt").unwrap();
    println!("Most Calories: {}", tasks::one(&data));
    println!("Sum of Top 3: {}", tasks::two(&data));
}


