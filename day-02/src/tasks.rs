pub mod one {
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
}

pub mod two {
    pub fn run(data: &String) -> i32 {
        // init variables
        let mut total: i32 = 0;
        let mut op_choice: i32;
        let mut my_choice: i32;
    
        // loop over the lines
        for line in data.lines() {
            op_choice = get_choice(line.chars().next().unwrap());    
            my_choice = get_choice(line.chars().last().unwrap());
            match my_choice {
                0 => total += if op_choice - 1 == 0 { 3 } else { op_choice - 1 },
                3 => total += op_choice,
                6 => total += if op_choice + 1 == 4 { 1 } else { op_choice + 1 },
                _ => total += 0,
            }
            total += my_choice;
        }
    
        // done!
        total 
    }
    
    fn get_choice(character: char) -> i32 {
        match character.to_uppercase().next().unwrap() {
            'A' => return 1,
            'B' => return 2,
            'C' => return 3,
            'X' => return 0,
            'Y' => return 3,
            'Z' => return 6,
            _ => return 0,
        }
    }
}