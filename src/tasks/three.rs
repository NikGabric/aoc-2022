use std::collections::HashSet;
use std::fs;

pub fn part_one() {
    let file_path = "./src/tasks/inputs/three.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let priorities_sum: u32 = text
        .lines()
        .map(|rucksack| {
            let (com1, com2) = rucksack.split_at(rucksack.len() / 2);

            com1
                .chars()
                .filter(|&c| com2.contains(c))
                .next()
                .and_then(eval_item)
                .unwrap_or(0)
        })
        .sum();

    println!("{priorities_sum}");
}

fn eval_item(item: char) -> Option<u32> {
    match item {
        'a'..='z' => Some(item as u32 -'a' as u32 + 1),
        'A'..='Z' => Some(item as u32 - 'A' as u32 + 27),
        _ => None
    }
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/three.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let rucksacks: Vec<&str> = text.lines().collect();
    let groups: Vec<_> = rucksacks.chunks(3).collect();
    let mut priorities_sum: u32 = 0;

    for group in groups {
        let mut chars: Vec<char> = group[0].chars().collect();
        chars.retain(|&c| group[1].contains(c) && group[2].contains(c));

        priorities_sum += eval_item(chars[0]).unwrap_or(0);
    }

    println!("{priorities_sum}");
}
