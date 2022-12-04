mod task_one;
mod task_two;

pub fn run(data: String) {
    println!("Total Subsets: {}", task_one::run(&data));
    println!("Total Overlaps: {}", task_two::run(&data));
}
