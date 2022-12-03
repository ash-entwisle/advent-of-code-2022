use std::collections::HashSet;

pub fn run(data: &String) -> i32 {
    // init variables
    let mut total: i32 = 0;

    // loop over the lines
    for line in data.lines() {
        
        // init hashset for line
        let mut set: HashSet<char> = HashSet::new();
        
        // loop over the characters
        for (index, character) in line.chars().enumerate() {

            // if the character is not in the set, add it
            if index < line.len() / 2 {
                set.insert(character);
            } else {
                // if the character is in the set, increse total by the priority
                if set.contains(&character) {
                    total += get_priority(character);
                    break;
                }
            }
        }
    }

    total
}

fn get_priority(item: char) -> i32 {
    if item.is_uppercase() {
        return item as i32 - 38;
    } else {
        return item as i32 - 96;
    }
}
