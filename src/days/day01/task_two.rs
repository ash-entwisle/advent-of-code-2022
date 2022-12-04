
pub fn run(data: &String) -> u32 {  
    let mut data = data.split("\r\n\r\n")                               
        .map(|group| group.lines()                      
            .map(|line| line.parse::<u32>().unwrap())   
            .sum())                                          
        .collect::<Vec<u32>>();                         
    data.sort();                                        
    data.into_iter().rev().take(3).sum::<u32>()         
}
