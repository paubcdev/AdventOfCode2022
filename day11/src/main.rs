// COPIED SOLUTION FROM: https://github.com/mheidal/rust-advent-of-code-2022/blob/master/src/day_11.rs

use std::collections::HashMap;
use regex::Regex;

struct Monkey {
    id: i64,
    items: Vec<i64>,
    operation: Box<dyn Fn (i64) -> i64>,
    div_test: i64,
    true_target: i64,
    false_target: i64,
    should_divide_by_3: bool,
    master_modulo: Option<i64>,
    interact_count: i64
}

impl Monkey {
    fn take_turn(&mut self) -> HashMap<i64, Vec<i64>>{
        // return map from monkey id to list of items to be passed to that monkey
        let mut target_monkeys = HashMap::<i64, Vec<i64>>::new();
        for item in &self.items {
            self.interact_count += 1;
            let mut new_val: i64 = (self.operation)(*item);
            if self.should_divide_by_3 {
                new_val = new_val / 3;
            }
            new_val = match self.master_modulo {
                Some(modulo) => new_val % modulo,
                None => new_val,
            };
            if new_val % self.div_test == 0 {
                target_monkeys
                    .entry(self.true_target)
                    .or_insert(vec![])
                    .push(new_val);
            } else {
                target_monkeys
                    .entry(self.false_target)
                    .or_insert(vec![])
                    .push(new_val);
            }
        }
        self.items = vec![];
        target_monkeys
    }
}

fn get_first_int(string: &str) -> i64 {
    let re = Regex::new(r"\d+").unwrap();
    for capture in re.captures_iter(string) {
        return capture[0].parse().unwrap();
    }
    panic!("No number found")
}

fn get_all_ints(string: &str) -> Vec<i64>{
    let re = Regex::new(r"\d+").unwrap();
    let mut numbers = Vec::new();
    for capture in re.captures_iter(string) {
        let number: i64 = capture[0].parse().unwrap();
        numbers.push(number);
    }
    numbers
}

fn get_operation(operation_str: &str) -> Box<dyn Fn (i64) -> i64> {
    let operation_segment = operation_str.trim().split(" = ").collect::<Vec<&str>>()[1];
    match operation_segment.split(' ').collect::<Vec<&str>>()[..] {
        ["old", op, "old"] => {
            match op {
                "*" => Box::new(move |old: i64| old * old),
                "+" => Box::new(move |old: i64| old + old),
                _ => panic!(),
            }
        },
        ["old", op, num] => {
            let val = num.parse::<i64>().expect("Number");
            match op {
                "*" => Box::new(move |old: i64| old * val),
                "+" => Box::new(move |old: i64| old + val),
                _ => panic!(),
            }
        },
        _ => panic!(),
    }
}

fn do_rounds(round_count: i32, should_divide_by_3: bool) -> Vec<Monkey> {
    let input = include_str!("../../inputs/day11.txt");
    let mut monkeys: Vec<Monkey> = vec![];
    let mut has_next: bool = true;
    let mut lines = input.split("\n");
    while has_next {
        match lines.next() {
            Some(line) => {
                let id = get_first_int(line);
                let items = get_all_ints(lines.next().expect("Items line"));
                let operation = get_operation(lines.next().expect("Operation line"));
                let div_test = get_first_int(lines.next().expect("Test line"));
                let true_target = get_first_int(lines.next().expect("True target line"));
                let false_target = get_first_int(lines.next().expect("False target line"));
                lines.next().expect("Blank line");
                let m = Monkey {
                    id, items, operation, div_test, true_target, false_target, should_divide_by_3, master_modulo: None, interact_count: 0
                };
                monkeys.push(m);
            },
            None => has_next = false,
        }
    }
    if !should_divide_by_3 {
        let master_modulo = monkeys
            .iter()
            .map(|m| m.div_test)
            .product();
        for monkey in &mut monkeys {
            monkey.master_modulo = Some(master_modulo)
        }
    }

    monkeys.sort_by(|a, b| a.id.cmp(&b.id));

    for _ in 0..round_count {
        // map of target IDs to items thrown to that target
        let mut targets_items: HashMap<i64, Vec<i64>> = HashMap::new();

        for thrower in &mut monkeys {
            // if any items have already been thrown to this monkey this round, extend the monkey's
            // list of items with those items
            thrower.items.extend(targets_items
                .get(&thrower.id)
                .unwrap_or(&vec![])
            );
            // clear list of items already thrown to this monkey this round
            targets_items.insert(thrower.id, vec![]);

            // populate lists in map
            for (target, items) in thrower.take_turn() {
                targets_items
                    .entry(target)
                    .or_insert(vec![])
                    .extend(items);
            }
        }
        // after all monkeys have thrown, all monkeys catch items thrown to them by throwers which
        // follow them in turn order
        for (target, items) in targets_items {
            monkeys
                .iter_mut()
                .filter(|m|m.id == target)
                .next()
                .expect("Target monkey")
                .items
                .extend(items)
        }
    }
    monkeys
}

fn get_monkey_business(mut monkeys: Vec<Monkey>) -> i64 {
    monkeys.sort_by(|a, b| b.interact_count.cmp(&a.interact_count));
    monkeys
        .into_iter()
        .take(2)
        .map(|m| m.interact_count)
        .product::<i64>()
}

fn part_1() {
    let monkeys = do_rounds(20, true);
    println!("Part 1: {}", get_monkey_business(monkeys));
}

fn part_2() {
    let monkeys = do_rounds(10000, false);
    println!("Part 2: {}", get_monkey_business(monkeys));
}

fn main () {
    part_1();
    part_2();
}