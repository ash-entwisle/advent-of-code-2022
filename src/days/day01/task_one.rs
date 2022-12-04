
pub fn run(data: &String) -> u32 {    
    data.split("\r\n\r\n")                              // split into groups
        .map(|group| group.lines()                      // for each line in each group
            .map(|line| line.parse::<u32>().unwrap())   // parse each line into a u32
            .sum())                                     // sum the lines
        .max().unwrap()                                 // get the max group                                                
}
