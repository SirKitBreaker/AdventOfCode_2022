use std::fs;

pub fn day1_part1_part2() {
    // Temp sum value
    let mut sum: u32 = 0;
    // Get input from file
    let x = fs::read_to_string("src/day1/input.txt").expect("File could not be read");
    let lines = x.lines();

    // Tuple to store the calories that each elf is carrying
    let mut calorie_counts: Vec<u32> = vec![];

    for s in lines {
        // Add each number
        if s != "" {
            sum = sum + s.parse::<u32>().unwrap();
        } else if s == "" {
            calorie_counts.push(sum);
            sum = 0;
            continue;
        }
    }

    calorie_counts.sort_by(|a, b| b.cmp(a));

    // Strongest elf
    let top_one = &calorie_counts[0];

    // Strongest three elves
    let top_three = &calorie_counts[0] + &calorie_counts[1] + &calorie_counts[2];

    println!("Most food carried: {}, Top three: {}", top_one, top_three);
}
