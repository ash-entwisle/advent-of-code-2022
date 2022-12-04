mod task_one;
mod task_two;

pub fn run(data: String) {
    println!("Task One: {}", task_one::run(&data));
    println!("Task Two: {}", task_two::run(&data));
}
