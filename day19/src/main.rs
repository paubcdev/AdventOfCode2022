mod part1;
mod part2;

use part1::*;
use part2::*;

fn main() {
    main1();
    main2()
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_solver(DATA), 33);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2_solver(DATA), 62);
    }
}