use std::fs;

pub fn part_one() {
    let file_path = "./src/tasks/inputs/five.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let (stacks, instructions) = text
        .split_once("\n\n")
        .unwrap();

    let mut grid = vec![vec![' '; 64]; 9];

    stacks
        .lines()
        .rev()
        .skip(1)
        .enumerate()
        .for_each(|(i, line)| {
            let crates = line
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<String>>()
                .iter()
                .enumerate()
                .for_each(|(j, el)| {
                    grid[j][i] = (el.chars().collect::<Vec<_>>()[1]);
                });
        });

    instructions
        .lines()
        .for_each(|line| {
            let line_split = line.split_whitespace().collect::<Vec<_>>();
            let mut amount = line_split[1].parse::<i32>().unwrap_or(0);
            let from = line_split[3].parse::<usize>().unwrap_or(0) - 1;
            let to = line_split[5].parse::<usize>().unwrap_or(0) - 1;

            while amount > 0 {
                if let Some(mut index_moving) = grid[from]
                    .iter()
                    .rev()
                    .position(|&c| c != ' ') {
                    if let Some(index) = grid[to].iter().position(|&c| c == ' ') {
                        index_moving = 63 - index_moving;
                        grid[to][index] = grid[from][index_moving];
                        grid[from][index_moving] = ' ';
                    }
                }
                amount -= 1;
            }
        });

    for col in grid {
        let char = col
            .iter()
            .rev()
            .find(|&&c| c != ' ')
            .unwrap_or(&' ');
        print!("{}", char);
    }
    println!()
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/one.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    println!(" ")
}
