use std::fs;
use std::ops::AddAssign;
use std::str::Lines;

// A -> Rock -> 1 | B -> Paper -> 2 | C -> Scissors -> 3
// X -> Rock -> 1 | Y -> Paper -> 2 | Z -> Scissors -> 3
// Win -> 6 | Draw -> 3 | Lose -> 0

pub fn day2_part1_part2() {
    // Total score
    let score:u32 = 0;
    // Get input from file
    let x = fs::read_to_string("src/day2/input.txt").expect("File could not be read");
    let lines = x.lines();

    day2_part1(&lines, score); // Part 1 function
    day2_part2(&lines, score); // Part 2 function
}

// Part 1 function
fn day2_part1(lines:&Lines, mut score:u32) {
    // Read each line
    for line in lines.to_owned() {
        // Get each hand thrown
        let thrown:Vec<String> = line.split(' ').map(|a| a.to_string()).collect();

        match thrown.as_slice() {
            [c1, c2] => 
            if (c1.eq("A") && c2.eq("X")) || (c1.eq("B") && c2.eq("Y")) || (c1.eq("C") && c2.eq("Z")) {
                score.add_assign(3); // Score for a draw
                score = add_player_score(score, c2);
            } else if c1.eq("A") && c2.eq("Y") || c1.eq("B") && c2.eq("Z") || c1.eq("C") && c2.eq("X") {
                score.add_assign(6); // Score for a win
                score = add_player_score(score, c2);
            } else if c1.eq("A") && c2.eq("Z") || c1.eq("B") && c2.eq("X") || c1.eq("C") && c2.eq("Y") {
                score.add_assign(0); // Score for a loss
                score = add_player_score(score, c2);
            },
            _ =>  {
                println!("End of input");
                break;
            }
        }
    }

    // Print result
    println!("Part 1 result: {}", score);
}

fn day2_part2(lines:&Lines, mut score:u32) {
    for line in lines.to_owned() {
        let thrown:Vec<String> = line.split(' ').map(|a| a.to_string()).collect();

        match thrown.as_slice() {
            [c1, c2] => 
            if c2.eq("Y") {
                score.add_assign(3); // Score for a draw
                match c1.as_str() {
                    "A" => score.add_assign(1),
                    "B" => score.add_assign(2),
                    "C" => score.add_assign(3),
                    _ => println!("Nope")
                };
            } else if c2.eq("Z") {
                score.add_assign(6); // Score for a win"
                match c1.as_str() {
                    "A" => score.add_assign(2),
                    "B" => score.add_assign(3),
                    "C" => score.add_assign(1),
                    _ => println!("Nope")
                };
            } else if c2.eq("X") {
                score.add_assign(0); // Score for a loss
                match c1.as_str() {
                    "A" => score.add_assign(3),
                    "B" => score.add_assign(1),
                    "C" => score.add_assign(2),
                    _ => println!("Nope")
                };
            },
            _ =>  {
                println!("End of input");
                break;
            }
        }
    }
    println!("Part 2 result: {}", score);
}

fn add_player_score(mut score:u32, choice:&String) -> u32 {
    match choice.as_str() {
        "X" => score.add_assign(1),
        "Y" => score.add_assign(2),
        "Z" => score.add_assign(3),
        _ => println!("Invalid choice")
    }

    return score;
}