mod solver;
use solver::*;

fn part_1_solver(input: &str) -> i64 {
    decrypt(input, 1, 1)
}

fn part_2_solver(input: &str) -> i64 {
    decrypt(input, 811589153, 10)
}

fn main(){
    let input = include_str!("../../inputs/day20.txt");
    println!("Part 1: {}", part_1_solver(input));
    println!("Part 2: {}", part_2_solver(input));
}

#[cfg(test)]
mod tests{
    use super::*;

    const DATA: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_solver(DATA), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_solver(DATA), 1623178306);
    }
}