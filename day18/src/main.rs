mod part1;
mod part2;

use part1::*;
use part2::*;

fn main() {
    main_part_1();
    main_part_2();
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_solver(DATA.trim()), 64);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_solver(DATA.trim()), 58);
    }
}