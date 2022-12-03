mod task_one;
mod task_two;

pub fn run(data: String) {
    println!("Total Score: {}", task_one::run(&data));
    println!("Total Score: {}", task_two::run(&data));
}
