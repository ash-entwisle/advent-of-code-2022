
pub fn run(data: &String) -> i32 {
    
    // init variables
    let mut total: i32 = 0;

    // loop over the lines
    for line in data.lines() {
        
        // slice the line in half
        let (left, right) = line.split_at(line.len() / 2);

        // if character in left and right, increase total by priority
        for character in left.chars() {
            if right.contains(character) {
                total += get_priority(character);
                break;
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
