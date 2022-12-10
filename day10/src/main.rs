fn main() {
    let input = include_str!("../../inputs/day10.txt");
    println!("Part 1: {}", solve_part_1(input))
}

struct CpuState {
    cycle: u32,
    x_reg: isize,
}

fn should_monitor_cycle(cycle_num: u32) -> bool {
    [20, 60, 100, 140, 180, 220].contains(&cycle_num)
}

fn solve_part_1(input: &str) -> isize {
    input
        .lines().map(|l| {
            l.split_whitespace()
                .skip(1)
                .next()
                .map(|e| e.parse::<isize>().unwrap())
        }).fold((CpuState{cycle: 1, x_reg: 1}, 0), |mut acc, curr| match curr {
            None => {
                let cycle_num = &mut acc.0.cycle;
                *cycle_num += 1;
                if should_monitor_cycle(*cycle_num) {
                    acc.1 += *cycle_num as isize * acc.0.x_reg;
                }
                acc
            }

            Some(delta) => {
                for i in 0..2 {
                    let cycle_num = &mut acc.0.cycle;
                    *cycle_num += 1;
                    if i == 1 {
                        acc.0.x_reg += delta;
                    }
                    if should_monitor_cycle(*cycle_num) {
                        acc.1 += *cycle_num as isize * acc.0.x_reg;
                    }
                }
                acc
            }
        })
        .1
}



#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(DATA), 13140);
    }
}