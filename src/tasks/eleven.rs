use std::fs;

#[derive(Debug)]
struct Monkey {
    id: i32,
    items: Vec<u64>,
    op_str: String,
    test: Test,
    inspect_amt: i32,
}
impl Monkey {
    fn new() -> Self {
        Self {
            id: 0,
            items: vec![],
            op_str: String::new(),
            test: Test::new(),
            inspect_amt: 0,
        }
    }

    fn modify_item(&mut self, index: usize, new_value: u64) {
        self.items[index] = new_value;
    }

    fn divide_by_3(&mut self, index: usize) {
        let val = self.items[index].clone();
        self.items[index] = val / 3;
    }

    fn do_operation(&mut self, index: usize) {
        let parsed_str = self.op_str.replace("new = old ", "");
        let (op, val_str) = parsed_str.split_once(" ").unwrap();
        let val: u64 = match val_str {
            "old" => self.items[index],
            _ => val_str.parse::<u64>().unwrap(),
        };
        match op {
            "+" => self.modify_item(index, self.items[index] + val),
            "-" => self.modify_item(index, self.items[index] - val),
            "*" => self.modify_item(index, self.items[index] * val),
            "/" => self.modify_item(index, self.items[index] / val),
            _ => (),
        }
    }
} 

#[derive(Debug)]
struct Test {
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}
impl Test {
    fn new() -> Self {
        Self {
            divisible_by: 0,
            if_true: 0,
            if_false: 0,
        }
    }

    fn execute_test(&self, worry_lvl: u64) -> usize {
        if (worry_lvl % self.divisible_by) == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

pub fn part_one() {
    let file_path = "./src/tasks/inputs/eleven.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let mut monkeys = init_monkeys(text);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let j = 0;
            while monkeys[i].items.len() >= 1 {
                monkeys[i].do_operation(j);
                monkeys[i].divide_by_3(j);
                let throw_to = monkeys[i].test.execute_test(monkeys[i].items[j]);
                let item = monkeys[i].items[j].clone();
                monkeys[throw_to].items.push(item);
                monkeys[i].items.remove(j);
                monkeys[i].inspect_amt += 1;
            }
        }
    }

    let amounts: Vec<_> = monkeys.iter().map(|m| m.inspect_amt).collect();
    let max = amounts.iter().max().unwrap();
    let second_max = amounts.iter().filter(|&&x| x != *max).max().unwrap();
    println!("{}", max * second_max)
}

pub fn part_two() {
    let file_path = "./src/tasks/inputs/eleven.txt";
    let text = fs::read_to_string(file_path)
        .expect("Not able to read the file");

    let mut monkeys = init_monkeys(text);

    let factor = monkeys.iter().map(|x| x.test.divisible_by).product::<u64>();

    for z in 0..10_000 {
        for i in 0..monkeys.len() {
            let j = 0;
            while monkeys[i].items.len() >= 1 {
                monkeys[i].do_operation(j);
                monkeys[i].items[j] %= factor;
                monkeys[i].inspect_amt += 1;
                let item = monkeys[i].items[j].clone();
                let throw_to = monkeys[i].test.execute_test(item);
                monkeys[throw_to].items.push(item);
                monkeys[i].items.remove(j);
            }
        }
    }

    let amounts: Vec<_> = monkeys.iter().map(|m| m.inspect_amt).collect();
    let max = amounts.iter().max().unwrap();
    let second_max = amounts.iter().filter(|&&x| x != *max).max().unwrap();
    println!("{}", *max as u128 * *second_max as u128);
}

fn init_monkeys(text: String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    let monkey_strings = text
        .split("\n\n")
        .collect::<Vec<_>>();

    for monkey in &monkey_strings {
        monkeys.push(parse_monkey(monkey));
    }

    monkeys
}

fn parse_monkey(monkey: &str) -> Monkey {
    let mut m: Monkey = Monkey::new();
    monkey
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            match i {
                0 => m.id = line.split_once(" ").unwrap().1.replace(":", "").parse::<i32>().unwrap_or(0),
                1 => m.items = line.replace("  Starting items: ", "").split(", ").map(|num| num.parse::<u64>().unwrap_or(0)).collect(),
                2 => m.op_str = line.replace("  Operation: ", ""),
                3 => m.test.divisible_by = line.replace("  Test: divisible by ", "").parse::<u64>().unwrap_or(0),
                4 => m.test.if_true = line.replace("    If true: throw to monkey ", "").parse::<usize>().unwrap_or(0),
                5 => m.test.if_false = line.replace("    If false: throw to monkey ", "").parse::<usize>().unwrap_or(0),
                _ => {}
            }

        });

    m
}
