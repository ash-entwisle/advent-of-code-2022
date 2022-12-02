use std::fs;

fn main() {
    // read the file
    let data: String = fs::read_to_string("./src/data.txt").unwrap();

    // init variables
    let mut max: i32 = 0;
    let mut num: i32 = 0;

    // loop over the lines
    for line in data.lines() {
        if !line.is_empty() {
            num += line.parse::<i32>().unwrap();
        } else {
            if num > max {
                max = num;
            }
            num = 0;
        }
    }

    // done!
    println!("sum: {}", max);
}
