use std::fs;



pub fn part_one() {
    let file_path = "./src/tasks/inputs/two.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");
    let games = text.split("\n");

    let mut score = 0;

    for game in games {
        let game: Vec<&str> = game.split_whitespace().collect();
        score += eval_game(game);
    }
    println!("{score}");
}

fn eval_game(game: Vec<&str>) -> i32 {
    let me = game[1];
    let enemy = game[0];
    let mut temp_score : i32 = match me {
        "X" => {
            1 + match enemy {
                "A" => 3,
                "B" => 0,
                "C" => 6,
                _ => 0,
            }
        },
        "Y" => {
            2 + match enemy {
                "A" => 6,
                "B" => 3,
                "C" => 0,
                _ => 0,
            }
        }
        "Z" => {
            3 + match enemy {
                "A" => 0,
                "B" => 6,
                "C" => 3,
                _ => 0,
            }
        },
        _ => 0
    };

    temp_score
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/two.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");
    let games = text.split("\n");

    let mut score = 0;

    for game in games {
        let game: Vec<&str> = game.split_whitespace().collect();
        score += predict_game(game);
    }
    println!("{score}");
}

fn predict_game(game: Vec<&str>) -> i32 {
    let me = game[1];
    let enemy = game[0];
    let mut temp_score : i32 = match me {
        "X" => {
            0 + match enemy {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => 0,
            }
        },
        "Y" => {
            3 + match enemy {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => 0,
            }
        }
        "Z" => {
            6 + match enemy {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _ => 0,
            }
        },
        _ => 0
    };

    temp_score
}