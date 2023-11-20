use std::collections::HashSet;
use std::fs;

pub fn part_one() {
    let file_path = "./src/tasks/inputs/six.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");
    let group_len = 4;
    let index = find_first_non_duplicate_group(text, group_len);

    println!("index: {}", index)
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/six.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");
    let group_len = 14;
    let index = find_first_non_duplicate_group(text, group_len);

    println!("index: {}", index)
}

fn find_first_non_duplicate_group(text: String, group_len: usize) -> usize {
    let mut i = 0;
    let mut group = &text[i..i + group_len];
    while check_for_duplicates(&group.chars().collect::<Vec<_>>()) {
        i += 1;
        group = &text[i..i + group_len];
    }

    i + group_len
}

fn check_for_duplicates(chars: &[char]) -> bool {
    let mut seen = HashSet::new();
    for c in chars {
        if !seen.insert(c) {
            return true;
        }
    }
    false
}
