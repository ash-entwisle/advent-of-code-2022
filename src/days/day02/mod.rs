mod task_one;
mod task_two;

pub fn run(data: String) {
    println!("Score: {}", task_one::run(&data));
    println!("Score: {}", task_two::run(&data));
}
