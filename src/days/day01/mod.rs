mod task_one;
mod task_two;

pub fn run(data: String) {
    println!("Most Calories: {}", task_one::run(&data));
    println!("Least Calories: {}", task_two::run(&data));
}
