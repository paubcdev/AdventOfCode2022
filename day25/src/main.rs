fn todec(s: &str) -> usize {
    s.chars().fold(0, |n, d| n * 5 + "=-012".chars().position(|x| x == d).unwrap() - 2)
}

fn tosnafu(n: usize) -> String {
    if n == 0 {String::new()} else {tosnafu((n+2)/5) + ["0","1","2","=","-"][n % 5]}
}

fn main() {
    let input = include_str!("../../inputs/day25.txt");
    println!("Part 1: {:?}", tosnafu(input.lines().map(todec).sum()))
}