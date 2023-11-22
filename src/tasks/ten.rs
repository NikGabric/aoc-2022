use std::fs;

pub fn part_one() {
    let file_path = "./src/tasks/inputs/ten.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let mut x: i32 = 1;
    let mut cycle: i32 = 0;
    let mut signal_strenght: i32 = 0;

    for line in text.lines() {
        if line.starts_with("addx") {
            let val = line.split_whitespace().collect::<Vec<_>>()[1].parse::<i32>().unwrap();
            cycle += 1;
            if should_update_signal_strength(cycle) {
                signal_strenght += cycle * x;
            }
            cycle += 1;
            if should_update_signal_strength(cycle) {
                signal_strenght += cycle * x;
            }
            x += val;
        } else {
            cycle += 1;
            if should_update_signal_strength(cycle) {
                signal_strenght += cycle * x;
            }
        }
    }
    println!("{signal_strenght}");
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/ten.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");
    println!();

    let mut x: i32 = 1;
    let mut cycle: i32 = 0;

    for line in text.lines() {
        if line.starts_with("addx") {
            let val = line.split_whitespace().collect::<Vec<_>>()[1].parse::<i32>().unwrap();
            print_pixel(cycle, x);
            cycle += 1;
            if should_newline(cycle) {
                println!();
                cycle = 0;
            }
            print_pixel(cycle, x);
            cycle += 1;
            if should_newline(cycle) {
                println!();
                cycle = 0;
            }
            x += val;
        } else {
            print_pixel(cycle, x);
            cycle += 1;
            if should_newline(cycle) {
                println!();
                cycle = 0;
            }
        }
    }
}

fn should_update_signal_strength(cycle: i32) -> bool {
    (cycle - 20) % 40 == 0
}

fn should_newline(cycle: i32) -> bool {
    cycle % 40 == 0
}

fn print_pixel(cycle: i32, x: i32) {
    if cycle == x-1 || cycle == x || cycle == x+1 {
        print!("#");
    } else {
        print!(".");
    }
}

