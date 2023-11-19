use std::fs;

pub fn part_one() {
    let file_path = "./src/tasks/inputs/four.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let num_of_pairs = text
        .lines()
        .map(|l| l.split(",").collect::<Vec<&str>>())
        .filter(|pair| check_pair(pair.to_vec()))
        .count();

    println!("{num_of_pairs}");
}

fn check_pair(str_vec: Vec<&str>) -> bool {
    let (elf_one_start, elf_one_end) = parse_range(str_vec[0]);
    let (elf_two_start, elf_two_end) = parse_range(str_vec[1]);

    let range_one = elf_one_start..=elf_one_end;
    let range_two = elf_two_start..=elf_two_end;

    (range_one.contains(&elf_two_start) && range_one.contains(&elf_two_end))
        || (range_two.contains(&elf_one_start) && range_two.contains(&elf_one_end))
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/four.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let num_of_pairs = text
        .lines()
        .map(|l| l.split(",").collect::<Vec<&str>>())
        .filter(|pair| check_pair_overlap(pair.to_vec()))
        .count();

    println!("{num_of_pairs}");
}


fn check_pair_overlap(str_vec: Vec<&str>) -> bool {
    let (elf_one_start, elf_one_end) = parse_range(str_vec[0]);
    let (elf_two_start, elf_two_end) = parse_range(str_vec[1]);

    (elf_one_start..=elf_one_end).contains(&elf_two_start)
        || (elf_two_start..=elf_two_end).contains(&elf_one_start)
}

fn parse_range(input: &str) -> (u32, u32) {
    let mut iter = input
        .split("-")
        .map(|el| el.parse().unwrap_or(0));
    (iter.next().unwrap(), iter.next().unwrap())
}
