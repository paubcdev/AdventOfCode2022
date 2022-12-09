use std::collections::HashSet;
use std::cmp::*;

fn main() {
    let input = include_str!("../../inputs/day09.txt");
    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

fn solve_part_1(input: &str) -> i32 {
    let mut visited = HashSet::new();
    let mut head = (0_i64, 0_i64);
    let mut tail = (0_i64, 0_i64);

    visited.insert(tail);

    for line in input.trim().lines() {
        let mut tt = line.split(' ');
        let direction = tt.next().unwrap();
        let steps = tt.next().unwrap().parse::<i64>().unwrap();

        for _ in 0..steps {
            match direction {
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                _ => panic!(),
            }

            if head.0.abs_diff(tail.0).max(head.1.abs_diff(tail.1)) > 1 {
                match head.0.cmp(&tail.0) {
                    Ordering::Greater => tail.0 += 1,
                    Ordering::Less => tail.0 -= 1,
                    Ordering::Equal => {}
                }

                match head.1.cmp(&tail.1) {
                    Ordering::Greater => tail.1 += 1,
                    Ordering::Less => tail.1 -= 1,
                    Ordering::Equal => {}
                }
            }

            visited.insert(tail);
        }
    }

    visited.len() as i32
}

fn solve_part_2(input: &str) -> i32 {
    let mut seen = HashSet::new();
    let mut knots = [(0_i64, 0_i64); 10];

    seen.insert(knots[1]);

    for line in input.trim().lines() {
        let mut tt = line.split(' ');
        let direction = tt.next().unwrap();
        let steps = tt.next().unwrap().parse::<i64>().unwrap();

        for _ in 0..steps {
            match direction {
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                "L" => knots[0].0 -= 1,
                "R" => knots[0].0 += 1,
                _ => panic!(),
            }

            for i in 1..knots.len() {
                let head = knots[i - 1];
                let tail = knots[i];

                if head.0.abs_diff(tail.0).max(head.1.abs_diff(tail.1)) > 1 {
                    match head.0.cmp(&tail.0) {
                        Ordering::Greater => knots[i].0 += 1,
                        Ordering::Less => knots[i].0 -= 1,
                        Ordering::Equal => {}
                    }

                    match head.1.cmp(&tail.1) {
                        Ordering::Greater => knots[i].1 += 1,
                        Ordering::Less => knots[i].1 -= 1,
                        Ordering::Equal => {}
                    }
                }

                seen.insert(knots[9]);
            }
        }
    }

    seen.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const DATA_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part_1_data_1() {
        let part1 = solve_part_1(DATA_1);
        assert_eq!(part1, 13);
    }

    #[test]
    fn test_part_1_data_2() {
        let part1 = solve_part_1(DATA_2);
        assert_eq!(part1, 88);
    }

    #[test]
    fn test_part_2_data_1() {
        let part2 = solve_part_2(DATA_1);
        assert_eq!(part2, 1);
    }

    #[test]
    fn test_part_2_data_2() {
        let part2 = solve_part_2(DATA_2);
        assert_eq!(part2, 36);
    }
}