use std::fs;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    fn update(&mut self, h: Coords) {
        let dx = h.x - self.x;
        let dy = h.y - self.y;
        if (dx.abs() + dy.abs() > 1) && (dx.abs() > 1 || dy.abs() > 1) {
            self.x += dx.signum();
            self.y += dy.signum();
        }
    }
}

pub fn part_one() {
    let file_path = "./src/tasks/inputs/nine.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let mut h: Coords = Coords::new();
    let mut t: Coords = Coords::new();
    let mut visited: HashSet<Coords> = HashSet::new();
    visited.insert(Coords {x: t.x, y: t.y});

    for line in text.lines() {
        let split_line = line.split_once(" ").unwrap();
        let dir: &str = split_line.0;
        let mut amount: i32 = split_line.1.parse::<i32>().unwrap();
        while amount > 0 {
            match dir {
                "U" => h.y += 1,
                "R" => h.x += 1,
                "D" => h.y -= 1,
                "L" => h.x -= 1,
                _ => {}
            }

            t.update(h);
            visited.insert(t);

            amount -= 1;
        }
    }

    println!("{:?}", visited.len());
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/nine.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let mut snake: Vec<Coords> = vec![Coords::new(); 10];
    let mut visited: HashSet<Coords> = HashSet::new();
    visited.insert(snake[9]);

    for line in text.lines() {
        let split_line = line.split_once(" ").unwrap();
        let dir: &str = split_line.0;
        let mut amount: i32 = split_line.1.parse::<i32>().unwrap();
        while amount > 0 {
            match dir {
                "U" => snake[0].y += 1,
                "R" => snake[0].x += 1,
                "D" => snake[0].y -= 1,
                "L" => snake[0].x -= 1,
                _ => {}
            }

            for i in 1..snake.len() {
                let lead = snake[i-1].clone();
                snake[i].update(lead);
            }
            
            visited.insert(snake[9]);
            amount -= 1;
        }
    }

    println!("{}", visited.len());
}
