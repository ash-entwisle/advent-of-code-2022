
pub fn run(data: &String) -> i32 {
    data.lines()
        .map(|line| (
            get_choice(line.chars().next()), 
            get_choice(line.chars().last())
        ))
        .map(|(a, b)| b + match b { 
            0 => if a-1 == 0 { 3 } else { a-1 },
            3 => a,
            6 => if a+1 == 4 { 1 } else { a+1 },
            _ => 0, 
         })
        .sum()
}

fn get_choice(character: Option<char>) -> i32 {
    match character.unwrap() {
        'A' => return 1,
        'B' => return 2,
        'C' => return 3,
        'X' => return 0,
        'Y' => return 3,
        'Z' => return 6,
        _ => return 0,
    }
}
