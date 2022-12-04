use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2())
}

fn part1() -> i32 {
    let input:Vec<(usize,usize,usize,usize)> = include_str!("../../inputs/day04.txt")
        .lines()
        .map(|l| l
            .split(['-', ','])
            .map(|v| v.parse::<usize>().unwrap())
            .collect_tuple::<(_,_,_,_)>()
            .unwrap()
        )
        .filter(|(s1,e1,s2,e2)| 
            (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1)
        )
        .collect();

    let res1 = input.len();
    res1 as i32
}

fn part2() -> i32 {
    let input:Vec<(usize,usize,usize,usize)> = include_str!("../../inputs/day04.txt")
        .lines()
        .map(|l| l
            .split(&['-', ','][..])
            .map(|v| v.parse::<usize>().unwrap())
            .collect_tuple::<(_,_,_,_)>()
            .unwrap()
        )
        .filter(|(s1,e1,s2,e2)|
            (s1 <= s2 && e1 >= s2) || (s2 <= s1 && e2 >= s1)
        )
        .collect();

    let res2 = input.len();
    res2 as i32
}