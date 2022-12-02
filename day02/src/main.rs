fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i32 {
	let input = include_str!("../../inputs/day02.txt");
	let mut score1 = 0;
	for line in input.lines() {
		let parts = line.split(" ").collect::<Vec<&str>>();
		match parts[1]  {
			"X" => {
				score1 += 1;
				if parts[0] == "A" {
					score1 += 3;
				} else if parts[0] == "C" {
					score1 += 6;
				}
			},
			"Y" => {
				score1 += 2;
				if parts[0] == "B" {
					score1 += 3;
				} else if parts[0] == "A" {
					score1 += 6;
				}
			},
			"Z" => {
				score1 += 3;
				if parts[0] == "C" {
					score1 += 3;
				} else if parts[0] == "B" {
					score1 += 6;
				}
			},
			_ => unreachable!(),
		}
	}
    score1
}

fn part2() -> i32 {
	let input = include_str!("../../inputs/day02.txt");
	let mut score2 = 0;
	for line in input.lines() {
		let parts = line.split(" ").collect::<Vec<&str>>();
		match parts[1]  {
			"X" => {
				if parts[0] == "A" {
					score2 += 3;
				} else if parts[0] == "C" {
					score2 += 2;
				} else {
					score2 += 1;
				}
			},
			"Y" => {
				score2 += 3;
				if parts[0] == "B" {
					score2 += 2;
				} else if parts[0] == "A" {
					score2 += 1;
				} else {
					score2 += 3;
				}
			},
			"Z" => {
				score2 += 6;
				if parts[0] == "C" {
					score2 += 1;
				} else if parts[0] == "B" {
					score2 += 3;
				} else {
					score2 += 2;
				}
			},
			_ => unreachable!(),
		}
	}
    score2
}