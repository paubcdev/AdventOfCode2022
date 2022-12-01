fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i32 {
    let input = include_str!("../../inputs/day01.txt");
    let mut food = vec![];
    let mut count = 0;

    for line in input.lines() {
        if line.is_empty() {
            food.push(count);
            count = 0;
        } else {
            count += line.parse::<i32>().unwrap();
        }
    }
    food.sort();
    food.reverse();
    food[0]
}

fn part2() -> i32 {
    let input = include_str!("../../inputs/day01.txt");
    let mut food = vec![];
    let mut count = 0;

    for line in input.lines() {
        if line.is_empty() {
            food.push(count);
            count = 0;
        } else {
            count += line.parse::<i32>().unwrap();
        }
    }
    food.sort();
    food.reverse();
    let sum = food[0] + food[1] + food[2];
    sum
}