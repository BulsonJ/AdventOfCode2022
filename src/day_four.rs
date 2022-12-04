pub fn run() {
    let input = include_str!("../input/day4.txt");

    println!("PART 1: {}", part_one(input));
    println!("PART 2: {}", part_two(input));
}

struct ElfAssignment {
    lowest_section: i32,
    highest_section: i32,
}

impl ElfAssignment {

    fn width(&self) -> i32{
        self.highest_section - self.lowest_section
    }

    fn is_contained_by(&self, other: &Self) -> bool {
        self.lowest_section >= other.lowest_section && self.highest_section <= other.highest_section
    }

    fn overlaps(&self, other: &Self) -> Option<i32> {
        let total_width = self.width() + other.width();
        let combined_width = self.highest_section.max(other.highest_section) - self.lowest_section.min(other.lowest_section);
        if total_width < combined_width {
            return None;
        }
        Some(1+total_width-combined_width)
    }
}

impl From<&str> for ElfAssignment {
    fn from(s: &str) -> Self {
        let mut sections = s.split('-');
        ElfAssignment {
            lowest_section: sections.next().unwrap().parse::<i32>().unwrap(),
            highest_section: sections.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}

fn part_one(input: &str) -> i32 {
    input.lines().map(|line| {
        let (elf_one, elf_two) = line.split_once(',').unwrap();
        let elf_assignment_one = ElfAssignment::from(elf_one);
        let elf_assignment_two = ElfAssignment::from(elf_two);
        
        (elf_assignment_one.is_contained_by(&elf_assignment_two) || elf_assignment_two.is_contained_by(&elf_assignment_one)) as i32     
    }).sum::<i32>()
}

fn part_two(input: &str) -> i32 {
    input.lines().map(|line| {
        let (elf_one, elf_two) = line.split_once(',').unwrap();
        let elf_assignment_one = ElfAssignment::from(elf_one);
        let elf_assignment_two = ElfAssignment::from(elf_two);

        elf_assignment_one.overlaps(&elf_assignment_two).is_some() as i32
    }).sum::<i32>()
}
