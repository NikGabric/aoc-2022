use std::fs;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Debug)]
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
                "U" => {
                    h.y += 1;
                    if h.y > t.y + 1 {
                        t.y += 1;
                        t.x = h.x;
                        visited.insert(Coords {x: t.x, y: t.y});
                    }
                }
                "R" => {
                    h.x += 1;
                    if h.x > t.x + 1 {
                        t.x += 1;
                        t.y = h.y;
                        visited.insert(Coords {x: t.x, y: t.y});
                    }
                }
                "D" => {
                    h.y -= 1;
                    if h.y < t.y - 1 {
                        t.y -= 1;
                        t.x = h.x;
                        visited.insert(Coords {x: t.x, y: t.y});
                    }
                }
                "L" => {
                    h.x -= 1;
                    if h.x < t.x - 1 {
                        t.x -= 1;
                        t.y = h.y;
                        visited.insert(Coords {x: t.x, y: t.y});
                    }
                }
                _ => {}
            }
            amount -= 1;
        }
    }

    println!("{:?}", visited.len());
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/nine.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");
}
