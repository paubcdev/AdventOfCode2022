use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());

}

fn pocket_conversor(pocket: &str) -> i64 {
    pocket.chars().fold(0, |a, char| {
        a | (1
            << match char {
                'a'..='z' => char as i64 - 97,
                'A'..='Z' => char as i64 - 39,
                _ => unreachable!(),
            })
    })
}

fn part1() -> i64 {
    let input = include_str!("../../inputs/day03.txt");
    let sacks: Vec<_> = input.lines().collect();

    let mut priority = 0;

    for (pocket_a, pocket_b) in sacks.iter().map(|n| n.split_at(n.len() / 2)) {
        let pocket_a = pocket_conversor(pocket_a);
        let pocket_b = pocket_conversor(pocket_b);

        let duplicate = ((pocket_a & pocket_b) as f64).log2() as i64;

        priority += duplicate + 1;
    }
    priority
}

fn part2() -> i64 {
    let input = include_str!("../../inputs/day03.txt");
    let sacks: Vec<_> = input.lines().collect();

    let mut priority_new = 0;

    for (sack_a, sack_b, sack_c) in sacks.iter().tuples() {
        let sack_a = pocket_conversor(sack_a);
        let sack_b = pocket_conversor(sack_b);
        let sack_c = pocket_conversor(sack_c);

        let duplicate = ((sack_a & sack_b & sack_c) as f64).log2() as i64;

        priority_new += duplicate + 1;
    }
    priority_new
}