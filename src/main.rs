use std::fs;

fn main() {
    let file_path = "input/day1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("PART 1: {}", day1_part_one(&contents));
    println!("PART 2: {}", day1_part_two(&contents));
}

fn day1_part_one(input: &str) -> i32{
    let mut current_elf = 0;
    let mut elves = Vec::new();
    for line in input.split('\n') {
        if line == "\r" || line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
            continue;
        }
        let string = line.trim_end();
        let number = string.parse::<i32>().unwrap();

        current_elf += number;
    }
    elves.sort();
    elves.iter().rev().take(1).sum()
}

fn day1_part_two(input: &str) -> i32 {
    let mut current_elf = 0;
    let mut elves = Vec::new();
    for line in input.split('\n') {
        if line == "\r" || line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
            continue;
        }
        let string = line.trim_end();
        let number = string.parse::<i32>().unwrap();

        current_elf += number;
    }
    elves.sort();
    elves.iter().rev().take(3).sum()
}
