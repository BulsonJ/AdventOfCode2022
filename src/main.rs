use std::env;
use std::fs;

fn main() {
    day1();
}

fn day1() {
    println!("Day one");
    let file_path = "input/day1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut current_elf = 0;
    let mut elves = Vec::new();
    for line in contents.split('\n') {
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
    let biggest: i32 = elves.iter().rev().take(1).sum();
    println!("Elf:{biggest}");

    let three_biggest: i32 = elves.iter().rev().take(3).sum();
    println!("Three Biggest Elves:{three_biggest}");
}
