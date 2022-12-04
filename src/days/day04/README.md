# Advent of Code, Day 1 Task 1

## Brief

```
--- Day 4: Camp Cleanup ---

Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique ID number, and each Elf is assigned a range of section IDs.

However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).

For example, consider the following list of section assignment pairs:

2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8

For the first few pairs, this list means:

    Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
    The Elves in the second pair were each assigned two sections.
    The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.

This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:

.234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8

Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.

In how many assignment pairs does one range fully contain the other?


```

## Task 1

Task 1 seems pretty simple. We have a list of strings and we need to see if there are any characters that appear in both halves of the string. If there are, we need to add the priority of that character to a total. All we need to do is split the string in half, loop through all the characters in the left half and see if they appear in the right half. If they do, we add the priority of that character to the total.

### Process

1. loop through each line of the input file
2. split the line in half
3. loop through each character in the left half and see if it appears in the right half
4. if it does, add the priority of that character to the total

### Psuedocode

```py
for line in data:
    left, right = line.split(line.length / 2)
    for char in left:
        if char in right:
            total += priority(char)

# priority(char) returns the priority of the character
```

### Code

First we initialise total which will hold the total priority of all the characters that appear in both halves of the string. We then loop through each line of the input file. We split the line in half and then loop through each character in the left half. If the character appears in the right half, we add the priority of that character to the total. The `get_priority` function returns the priority of the character based on its ASCII value. If the character is lowercase, we subtract 96 from its ASCII value to get the priority. If the character is uppercase, we subtract 64 from its ASCII value to get the priority.

```rs
pub fn run(data: &String) -> i32 {
    // initialize total
    let mut total: i32 = 0;
    // loop through lines
    for line in data.lines() {
        // split line in half
        let (left, right) = line.split_at(line.len() / 2);
        // if there is a match, increase score by priority
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
```

## Part 2



### Process

1. 

### Psuedocode

```py
# TODO
```

### Code

// TODO

```rs
use std::collections::HashSet;

pub fn run(data: &String) -> i32 {
    // init variables
    let mut total: i32 = 0;
    let mut sets: Vec<HashSet<char>> = vec![
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
    ];
    // loop over the lines
    for (index, line) in data.lines().enumerate() {
        // set the coresponding set to a set of the current line
        match index % 3 {
            0 => { sets[index % 3] = line.chars().collect(); },
            1 => { sets[index % 3] = line.chars().collect(); },
            2 => { sets[index % 3] = line.chars().collect(); },
            _ => {}
        }
        // if the index is 3, check the sets
        if index % 3 == 2 {
            for character in sets[0].intersection(&sets[1]).cloned() {
                if sets[2] .contains(&character) {
                    total += get_priority(character);
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
```
