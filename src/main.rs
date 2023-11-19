#![allow(unused)]

mod tasks;

fn main() {
    let tasks_to_run = vec![
        tasks::one::part_one,
        tasks::one::part_two,
        tasks::two::part_one,
        tasks::two::part_two,
        tasks::three::part_one,
        tasks::three::part_two,
        tasks::four::part_one,
        tasks::four::part_two,
    ];

    let (mut day, mut part) = (1, 1);
    for task in &tasks_to_run {
        run_task(*task, day, part);
        if part == 2 {
            day += 1;
            part = 1;
        } else {
            part += 1;
        }
    }
}

fn run_task(func: fn(), day: i32, part: i32) {
    print!("Running day {}, part number {}: ", day, part);
    func();
}
