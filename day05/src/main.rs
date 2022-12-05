fn main() {
    let input = include_str!("../../inputs/day05.txt");

    println!("Part 1: {}", move_crates(input, false));

    println!("Part 2 (CRATEMOVER 9001): {}", move_crates(input, true));
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_crates(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::<Vec<char>>::with_capacity(9);

    input.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, item)| !item.is_ascii_whitespace())
            .for_each(|(i, item)| {
                if i >= stacks.len() {
                    let mut new_vec = Vec::with_capacity(10);
                    new_vec.push(item);
                    stacks.push(new_vec);
                } else {
                    stacks[i].push(item);
                }
            })
    });

    stacks
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (count, fromto) = line[5..].split_once(" from ")?;
            let (from, to) = fromto.split_once(" to ")?;
            Some(Instruction {
                count: count.parse::<usize>().ok()?,
                from: from.parse::<usize>().ok()?,
                to: to.parse::<usize>().ok()?,
            })
        })
}

fn do_movement(stacks: &mut Vec<Vec<char>>, instruction: &Instruction, cratemover_9001: bool) {
    let from_stack = stacks.get_mut(instruction.from - 1).unwrap();

    let mut crates = from_stack.split_off(from_stack.len() - instruction.count);

    if !cratemover_9001 {
        crates.reverse();
    }

    let to_stack = stacks.get_mut(instruction.to - 1).unwrap();
    to_stack.append(&mut crates);
}

fn move_crates(input: &str, cratemover_9001: bool) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let stacks = parse_crates(crates);
    let instructions = parse_instructions(instructions);

    let new_stacks = instructions.fold(stacks, |mut stacks, op| {
        do_movement(&mut stacks, &op, cratemover_9001);
        stacks
    });

    new_stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_example_part1() {
        let top = move_crates(EXAMPLE, false);

        assert_eq!(top, "CMZ");
    }

    #[test] 
    fn test_example_part2() {
        let top = move_crates(EXAMPLE, true);

        assert_eq!(top, "MCD");
    }
}