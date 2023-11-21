use std::fs;
use std::path::PathBuf;

use regex::Regex;

pub fn part_one() {
    let file_path = "./src/tasks/inputs/seven.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let mut cwd = String::new();
    let mut dirs: Vec<(String, i64)> = vec![(String::from("./"), 0)];

    text.lines().for_each(|line| {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        if line.starts_with("$ cd") {
            move_dir(&mut cwd, line_split[2]);
        } else if is_line_dir(line) {
            let dir_split: Vec<_> = line.split_whitespace().collect();
            dirs.push((format!("{}/{}", cwd, dir_split[1]), 0));
        } else if is_line_file(line) {
            let size: i64 = line.split_whitespace().collect::<Vec<_>>()[0].parse::<i64>().unwrap_or(0);
            if let Some((_, curr_size)) = dirs.iter_mut().find(|(dir, _)| *dir == cwd) {
                *curr_size += size;
            }
        }
    });

    let cloned_dirs = dirs.clone();

    dirs.iter_mut().for_each(|(curr_dir, curr_size)| {
        let pattern_str = format!(r"^{}.+$", regex::escape(curr_dir));
        let pattern = Regex::new(&pattern_str).unwrap();

        let sum_sizes: i64 = cloned_dirs
            .iter()
            .filter(|(dir, _)| pattern.is_match(dir))
            .map(|(_, size)| size)
            .sum();

        *curr_size += sum_sizes;
    });

    let sum: i64 = dirs
        .iter()
        .filter(|(_, size)| size <= &100000)
        .map(|(_, size)| size)
        .sum();

    println!("{sum}");
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/seven.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");
}

fn is_line_dir(line: &str) -> bool {
    let pattern = Regex::new(r"^dir [a-zA-z]*").unwrap();
    pattern.is_match(line)
}

fn is_line_file(line: &str) -> bool {
    let pattern = Regex::new(r"^\d+ .+$").unwrap();
    pattern.is_match(line)
}

fn move_dir(cwd: &mut String, to: &str) {
    match to {
        "/" => {
            *cwd = String::from(".");
        }
        ".." => {
            let mut path_buff = PathBuf::from(&*cwd);
            path_buff.pop();
            *cwd = path_buff.to_string_lossy().to_string();
        }
        dir => {
            *cwd += &*format!("/{dir}");
        }
    }
}

