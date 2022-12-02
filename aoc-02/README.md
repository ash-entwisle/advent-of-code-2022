# Advent of Code, Day 1 Task 1

## Brief

```
--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z

This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
    The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?

--- Part Two ---

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

    In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

```

## Task 1

For task 1, We need to calculate the total score. To do this we need to loop through the input and calculate the score for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

### Process

1. loop through each line of the input file
2. add points to the score based on what you play 
3. add points to the score depending on the outcome of the round

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
