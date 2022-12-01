use std::env;
use std::fs;

fn main() {
    // --snip--
    let file_path = "input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut elves = contents.split("\n\n");

    let mut calorieTotals = Vec::new();

    for elf in elves {
        let mut calories = 0;

        let mut items = elf.lines();
        for i in items {
            calories = calories + i.parse::<i32>().unwrap();
        }

        calorieTotals.push(calories);
    }
    calorieTotals.sort();
    calorieTotals.reverse();

    println!("Max Calories: {}", calorieTotals[0]);
    println!("Top 3 Sum: {}", calorieTotals[0]+calorieTotals[1]+calorieTotals[2]);
}