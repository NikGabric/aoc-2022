use std::fs;

pub fn part_one() {
    let file_path = "./src/tasks/inputs/eight.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let mut grid: Vec<Vec<i8>> = init_grid(text);
    let mut sum: i32 = (grid.len() * grid[0].len()) as i32;

    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            let mut visible: bool = true;
            // Up
            for k in 0..i {
                if grid[i][j] <= grid[k][j] {
                    visible = false;
                    break;
                }
            }
            if visible {
                continue;
            } else {
                visible = true;
            }
            // Right
            for k in (j + 1)..grid[i].len() {
                if grid[i][j] <= grid[i][k] {
                    visible = false;
                    break;
                }
            }
            if visible {
                continue;
            } else {
                visible = true;
            }
            // Down
            for k in (i + 1)..grid.len() {
                if grid[i][j] <= grid[k][j] {
                    visible = false;
                    break;
                }
            }
            if visible {
                continue;
            } else {
                visible = true;
            } 
            // Left
            for k in 0..j {
                if grid[i][j] <= grid[i][k] {
                    visible = false;
                    break;
                }
            }
            if !visible {
                sum -= 1;
            }
        }
    }

    println!("{sum}");
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/eight.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let mut grid = init_grid(text);
    let mut max: i32 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let (mut up, mut right, mut down, mut left) = (0, 0, 0, 0);
            // Up
            if i > 0 {
                for k in (0..i).rev() {
                    up += 1;
                    if grid[i][j] <= grid[k][j] {
                        break;
                    }
                }
            }
            // Right
            for k in (j + 1)..grid[i].len() {
                right += 1;
                if grid[i][j] <= grid[i][k] {
                    break;
                }
            }
            // Down
            for k in (i + 1)..grid.len() {
                down += 1;
                if grid[i][j] <= grid[k][j] {
                    break;
                }
            }
            // Left
            if j > 0 {
                for k in (0..j).rev(){
                    left += 1;
                    if grid[i][j] <= grid[i][k] {
                        break;
                    }
                }
            }
            let scenic_score = up * right * down * left;
            if scenic_score > max {
                max = scenic_score;
            }
        }
    }

    println!("{max}");
}

fn init_grid(text: String) -> Vec<Vec<i8>> {
    let mut grid: Vec<Vec<i8>> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if grid.len() < i + 1 {
            grid.push(vec![]);
        }
        line.chars().for_each(|char| grid[i].push((char.to_digit(10).unwrap()) as i8));
    }
    grid
}
