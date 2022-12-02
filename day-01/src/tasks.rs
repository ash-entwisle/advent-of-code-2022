
pub fn one(data: &String) -> i32 {
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

    // DONE!
    max
} 

pub fn two(data: &String) -> i32 {
    // init variables
    let mut totals: Vec<i32> = Vec::new();
    let mut num: i32 = 0;
    
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
    
    // DONE!
    totals[totals.len() - 3..].iter().sum()
}
