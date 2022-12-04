
pub fn run(data: &String) -> u32 {  
    let mut data = data.split("\r\n\r\n")               // split into groups                    
        .map(|group| group.lines()                      // for each line in each group
            .map(|line| line.parse::<u32>().unwrap())   // parse each line into a u32
            .sum())                                     // sum the lines     
        .collect::<Vec<u32>>();                         // collect into a vector
    data.sort();                                        // sort the vector  (rust... why?)
    data.into_iter().rev().take(3).sum::<u32>()         // return the sum of the 3 largest groups
}
