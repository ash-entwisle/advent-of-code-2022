use std::fs;

fn main() {
    // read the file
    let data: String = fs::read_to_string("./src/data.txt").unwrap();

    // init variables
    let mut num: i32 = 0;
    let mut totals: Vec<i32> = Vec::new();
    
    // loop over the lines
    for line in data.lines() {
        if !line.is_empty() {
            num += line.parse::<i32>().unwrap();
        } else {
            totals.push(num);
            num = 0;
        }
    }

    // sort vector and sum 3 largest
    totals.sort();
    let sum: i32 = totals[totals.len() - 3..].iter().sum();

    // done!
    println!("sum: {}", sum);
}