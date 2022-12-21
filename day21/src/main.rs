use std::{collections::HashMap, error::Error};


#[derive(Debug,Copy,Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

#[derive(Debug,Clone)]
struct CalculationResult {
    left: String,
    right: String,
    operation: Operation
}

#[derive(Debug,Clone)]
enum Monkey {
    Number(i64),
    CalcResult(CalculationResult)
}


fn main() ->Result<(), Box<dyn Error>>{
    let input = include_str!("../../inputs/day21.txt");
    let monkeys =  parse_data(input);
    println!("Part 1: {:?}", part_1_solver(&monkeys, "root".to_string()));
    println!("Part 2: {:?}", part_2_solver(&monkeys));
 
    Ok(())

}

fn parse_data(data: &str) -> HashMap<String, Monkey> {
    let mut result = HashMap::new();

    for line in data.lines() {
        let mut tokens = line.split(": ");
        let monkey_name = tokens.next().unwrap().to_owned();
        let output = tokens.next().unwrap();
        let parsed = if let Ok(n) = output.parse::<i64>() {
            Monkey::Number(n)
        } else {
            let mut tokens = output.split(' ');
            let left = tokens.next().unwrap().to_owned();
            let operation = match tokens.next() {
                Some("+") => Operation::Add,
                Some("-") => Operation::Subtract,
                Some("*") => Operation::Multiply,
                Some("/") => Operation::Divide,
                _ => unimplemented!()
            };
            let right = tokens.next().unwrap().to_owned();
            Monkey::CalcResult(CalculationResult{left, right, operation})
        };
        result.insert(monkey_name, parsed);
    }

    result
}

fn part_1_solver(monkeys: &HashMap<String, Monkey>, monkey: String ) -> i64{
    match monkeys.get(&monkey) {
        Some(Monkey::Number(n)) => *n,
        Some(Monkey::CalcResult(cal)) => {
            match cal.operation {
                Operation::Add => part_1_solver(monkeys, cal.left.clone()) + part_1_solver(monkeys, cal.right.clone()),
                Operation::Subtract => part_1_solver(monkeys, cal.left.clone()) - part_1_solver(monkeys, cal.right.clone()),
                Operation::Multiply => part_1_solver(monkeys, cal.left.clone()) * part_1_solver(monkeys, cal.right.clone()),
                Operation::Divide => part_1_solver(monkeys, cal.left.clone()) / part_1_solver(monkeys, cal.right.clone()),
            }
        }
        None => unimplemented!()
    }
}

fn part_2_solver(monkeys: &HashMap<String, Monkey>) -> i64 {
    if let Monkey::CalcResult(res) = monkeys.get("root").unwrap() {
        let left_total = part_1_solver(monkeys, res.left.to_owned());
        let right_total = part_1_solver(monkeys, res.right.to_owned());
        if is_human(monkeys, &res.left) {
            part2_inner(monkeys, &res.left, right_total)
        } else {
            part2_inner(monkeys, &res.right, left_total)
        }
    } else {
        unreachable!()
    }
}

fn part2_inner(monkeys: &HashMap<String, Monkey>, monkey: &str, desired: i64) -> i64 {
    if monkey == "humn" {
        desired
    } else {
        match monkeys.get(monkey).unwrap() {
            Monkey::CalcResult(res) => {
                if is_human(monkeys, &res.left) {
                    let right_result = part_1_solver(monkeys, res.right.clone());
                    let next_desired = match res.operation{
                        Operation::Add => desired - right_result,
                        Operation::Subtract => desired + right_result,
                        Operation::Multiply => desired / right_result,
                        Operation::Divide => desired * right_result,
                    };
                    part2_inner(monkeys, &res.left, next_desired)
                } else {
                    let left_result = part_1_solver(monkeys, res.left.clone());
                    let next_desired = match res.operation{
                        Operation::Add => desired - left_result,
                        Operation::Subtract => left_result - desired,
                        Operation::Multiply => desired / left_result,
                        Operation::Divide => left_result * desired,
                    };
                    part2_inner(monkeys, &res.right, next_desired)
                }
            },
            Monkey::Number(res) => {*res}
        }
    }
}

fn is_human(monkeys: &HashMap<String, Monkey>, monkey: &str) -> bool {
    if monkey == "humn" {
        true
    } else {
        match monkeys.get(monkey) {
            Some(Monkey::Number(_)) => false,
            Some(Monkey::CalcResult(cal)) => {
                is_human(monkeys, &cal.left) || is_human(monkeys, &cal.right)}
            None => unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part_1() {
        let monkeys =  parse_data(DATA);
        assert_eq!(part_1_solver(&monkeys, "root".to_string()), 152)
    }

    #[test]
    fn test_part_2() {
        let monkeys =  parse_data(DATA);
        assert_eq!(part_2_solver(&monkeys), 301)
    }
}
