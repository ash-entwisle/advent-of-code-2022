pub fn run(data: &String) -> i32 {
    // init variables
    let mut total: i32 = 0;
    let mut op_choice: i32;
    let mut my_choice: i32;

    // loop over the lines
    for line in data.lines() {
        op_choice = get_choice(line.chars().next().unwrap());   
        my_choice = get_choice(line.chars().last().unwrap());
        match op_choice - my_choice {
            -1 | 2 => total += 6,
            0 => total += 3,
            _ => total += 0,
        }
        total += my_choice;
    }

    // done!
    total
}

fn get_choice(character: char) -> i32 {
    match character.to_uppercase().next().unwrap() {
        'X'|'A' => return 1,
        'Y'|'B' => return 2,
        'Z'|'C' => return 3,
        _ => return 0,
    }
}