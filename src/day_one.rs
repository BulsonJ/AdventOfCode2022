fn day1_part_one(input: &str) -> i32 {
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
