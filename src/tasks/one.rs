use std::fs;

pub fn part_one() {
    let file_path = "./src/tasks/inputs/one.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");
    let elves = text.split("\n\n");

    let mut max = 0;

    for elf in elves {
        let sum: i32= elf.split("\n").map(|val| val.parse::<i32>())
            .filter_map(Result::ok)
            .sum();
        if sum > max {
            max = sum;
        }
    }

    println!("{max}");
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/one.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");
    let elves = text.split("\n\n");

    let mut top_three: Vec<i32> = vec![0; 3];

    for elf in elves {
        let sum: i32= elf.split("\n").map(|val| val.parse::<i32>())
            .filter_map(Result::ok)
            .sum();

        if sum > *top_three.first().unwrap() {
            top_three[0] = sum;
            top_three.sort_by(|a, b| a.cmp(b));
        }
    }

    println!("{}", top_three.iter().sum::<i32>());
}
