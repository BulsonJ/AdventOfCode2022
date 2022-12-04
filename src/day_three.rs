use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../input/day3.txt");

    println!("PART 1: {}", part_one(input));
    println!("PART 2: {}", part_two(input));
}

const BASE_ASCII_LOWERCASE_VALUE: i32 = 'a' as i32;
const BASE_ASCII_UPPERCASE_VALUE: i32 = 'A' as i32;

fn get_item_priority(input: char) -> Result<i32, ()> {
    if input.is_ascii_uppercase() {
        Ok(27 + input as i32 - BASE_ASCII_UPPERCASE_VALUE)
    } else if input.is_ascii_lowercase() {
        Ok(1 + input as i32 - BASE_ASCII_LOWERCASE_VALUE)
    } else {
        Err(())
    }
}

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|backpack| {
            let (compartment_one, compartment_two) = backpack.split_at(backpack.len() / 2);
            let mut used_characters = HashSet::new();
            for char in compartment_one.chars() {
                used_characters.insert(char);
            }

            let mut in_both_compartments = HashSet::new();
            for char in compartment_two.chars() {
                if used_characters.contains(&char) {
                    in_both_compartments.insert(char);
                }
            }

            in_both_compartments
                .iter()
                .map(|item| get_item_priority(*item).unwrap())
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn part_two(input: &str) -> i32 {
    let mut score = 0i32;

    let bags: Vec<&str> = input.lines().collect();
    for (count, bag) in bags.iter().enumerate().step_by(3usize) {
        let first_bag = bag;
        let &second_bag = bags.get(count + 1usize).unwrap();
        let &third_bag = bags.get(count + 2usize).unwrap();

        let mut used_characters = HashSet::new();
        for char in first_bag.chars() {
            used_characters.insert(char);
        }

        let mut in_both_bags = HashSet::new();
        for char in second_bag.chars() {
            if used_characters.contains(&char) {
                in_both_bags.insert(char);
            }
        }

        let mut in_all_bags = HashSet::new();
        for char in third_bag.chars() {
            if in_both_bags.contains(&char) {
                in_all_bags.insert(char);
            }
        }

        score += in_all_bags
            .iter()
            .map(|item| get_item_priority(*item).unwrap())
            .sum::<i32>();
    }
    score
}
