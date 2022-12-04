
pub fn run(data: &String) -> u32 {    
    data.split("\r\n\r\n")                              
        .map(|group| group.lines()                      
            .map(|line| line.parse::<u32>().unwrap())   
            .sum())                                     
        .max().unwrap()                                                                                 
}
