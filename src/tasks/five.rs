use std::fs;

pub fn part_one() {
    let file_path = "./src/tasks/inputs/five.txt";
    let (stacks, instructions) = read_input(file_path);

    let mut grid = initialize_grid(&stacks);

    process_instructions(&mut grid, &instructions);

    print_answer(grid);
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/five.txt";
    let (stacks, instructions) = read_input(file_path);

    let mut grid = initialize_grid(&stacks);

    process_instructions_two(&mut grid, &instructions);

    print_answer(grid);
}

fn read_input(file_path: &str) -> (String, String) {
    let text: String = fs::read_to_string(file_path)
        .expect("Not able to read the file");
    text
        .split_once("\n\n")
        .map(|(first, second)| (first.to_string(), second.to_string()))
        .unwrap()
}


fn initialize_grid(stacks: &str) -> Vec<Vec<char>> {
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
    grid
}

fn process_instructions(grid: &mut Vec<Vec<char>>, instructions: &String) {
    instructions
        .lines()
        .for_each(|line| {
            let (amount, from, to) = parse_instruction_line(line);
            move_crates(grid, amount, from, to);
        })
}

fn process_instructions_two(grid: &mut Vec<Vec<char>>, instructions: &String) {
    instructions
        .lines()
        .for_each(|line| {
            let (amount, from, to) = parse_instruction_line(line);
            move_crates_two(grid, amount, from, to);
        })
}

fn parse_instruction_line(line: &str) -> (usize, usize, usize) {
    let line_split: Vec<&str> = line.split_whitespace().collect();
    let amount = line_split[1].parse().unwrap_or(0);
    let from = line_split[3].parse().unwrap_or(0) - 1;
    let to = line_split[5].parse().unwrap_or(0) - 1;
    (amount, from, to)
}

fn move_crates(grid: &mut Vec<Vec<char>>, amount: usize, from: usize, to: usize) {
    for _ in 0..amount {
        if let Some(index_moving) = grid[from].iter().rev().position(|&c| c != ' ') {
            if let Some(index) = grid[to].iter().position(|&c| c == ' ') {
                grid[to][index] = grid[from][63 - index_moving];
                grid[from][63 - index_moving] = ' ';
            }
        }
    }
}

fn move_crates_two(grid: &mut Vec<Vec<char>>, amount: usize, from: usize, to: usize) {
    for _ in amount..0 {
        if let Some(index_moving) = grid[from].iter().rev().position(|&c| c != ' ') {
            if let Some(index) = grid[to].iter().position(|&c| c == ' ') {
                grid[to][index] = grid[from][64 - amount - index_moving];
                grid[from][64 - amount - index_moving] = ' ';
            }
        }
    }
}

fn print_answer(grid: Vec<Vec<char>>) {
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
