# Advent of Code, Day 1 Task 1

## Brief

```
--- Day 3: Rucksack Reorganization ---

One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

    The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
    The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
    The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
    The fourth rucksack's compartments only share item type v.
    The fifth rucksack's compartments only share item type t.
    The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?

--- Part Two ---

As you finish identifying the misplaced items, the Elves come to you with another issue.

For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, the first group's rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg

And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their badge item type must be Z.

Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?

```

## Task 1

// TODO

### Process

1. loop through each line of the input file
2. split the line
3. 

### Psuedocode

```py
def main(data):
    score = 0
    for line in data:
        # add points to the score based on what you play 
        # add points to the score depending on the outcome of the round
    return score
```

### Code

First the code reads the input file and initalizes 3 variables. `total` will track the total score, `my_choice` will track what the player plays and `op_choice` will track what the opponent plays. The code then loops through each line of the file and gets the first and last character. It then uses the `get_choice` function to convert ABC/XYZ to 123. I then use a switch statement to determine who wins. If the player wins, they get 6 points, if they lose they get 0 points and if it's a draw they get 3 points. The score is then added to the total score (i will add a more detailed explination of this later, test.txt has some more notes on how I figured it out).

```rs
use std::fs;

fn main() {
    // read the file
    let data: String = fs::read_to_string("./data.txt").unwrap();

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
    println!("Total: {}", total);
}

fn get_choice(character: char) -> i32 {
    match character.to_uppercase().next().unwrap() {
        'X'|'A' => return 1,
        'Y'|'B' => return 2,
        'Z'|'C' => return 3,
        _ => return 0,
    }
}


```

## Part 2

With part 2, we can use some of the same code as last time but edit it a bit. First we need to edit the `get_choice` function so it returns 0, 3 and 6 on X, Y and Z respectively. Then we need to edit the loop in the main function to check for wha the win condition is. 

### Process

1. loop through each line of the input file
2. convert first character to 1, 2 or 3
3. convert last character to 0, 3 or 6
4. if last character is 0, add first character -1 to total
5. if last character is 3, add first character to total
6. if last character is 6, add first character + 1 to total
7. add score to total

### Psuedocode

```py
total = 0
for line in data:
    # convert first character to 1, 2 or 3
    # convert last character to 0, 3 or 6
    # if last character is 0, add first character -1 to total
    # if last character is 3, add first character to total
    # if last character is 6, add first character + 1 to total
    # add score to total
return total   
```

### Code

This code is pretty much the same as part 1, except the function `get_choice` returns 0, 3 and 6 on X, Y and Z respectively. Then in the loop, I check if the last character is 0, 3 or 6 and add different values to the total score depending on what it is.  

```rs
use std::fs;

fn main() {
    // read the file
    let data: String = fs::read_to_string("./data.txt").unwrap();

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
    println!("Total: {}", total);
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
```
