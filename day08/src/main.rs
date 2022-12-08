fn main () {
    let input = include_str!("../../inputs/day08.txt");
    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

fn solve_part_1(data: &str) -> i32 {
	let lines: Vec<_> = data.lines().collect();
	let mut trees = Vec::new();
    let mut sum_vis = 0;

	for i in 0..lines.len() {
		let line: Vec<_> = lines[i].chars().collect();
		trees.push(Vec::new());
		for j in 0..line.len() {
			trees[i].push(line[j].to_digit(10).unwrap());
		}
	}

	for i in 0..trees.len() {
		for j in 0..trees[i].len() {
			let height = trees[i][j];
			let mut left = true;
			let mut right = true;
			let mut top = true;
			let mut bottom = true;
			for k in 0..i {
				if trees[k][j] >= height {
					top = false;
				}
			}
			for k in 0..j {
				if trees[i][k] >= height {
					left = false;
				}
			}
			for k in i+1..trees.len() {
				if trees[k][j] >= height {
					bottom = false;
				}
			}
			for k in j+1..trees[i].len() {
				if trees[i][k] >= height {
					right = false;
				}
			}
			let visible = left || right || top || bottom;
			if visible {
				sum_vis += 1;
			}
		}
	}
    sum_vis
}

fn solve_part_2(data: &str) -> i32 {
    let lines: Vec<_> = data.lines().collect();
    let mut trees = Vec::new();
    let mut highest = 0;

	for i in 0..lines.len() {
		let line: Vec<_> = lines[i].chars().collect();
		trees.push(Vec::new());
		for j in 0..line.len() {
			trees[i].push(line[j].to_digit(10).unwrap());
		}
	}

	for i in 0..trees.len() {
		for j in 0..trees[i].len() {
			let height = trees[i][j];
			let mut left = 0;
			let mut right = 0;
			let mut top = 0;
			let mut bottom = 0;
			for k in (0..i).rev() {
				top += 1;
				if trees[k][j] >= height {
					break;
				}
			}
			for k in (0..j).rev() {
				left += 1;
				if trees[i][k] >= height {
					break;
				}
			}
			for k in i+1..trees.len() {
				bottom += 1;
				if trees[k][j] >= height {
					break;
				}
			}
			for k in j+1..trees[i].len() {
				right += 1;
				if trees[i][k] >= height {
					break;
				}
			}
			let visible = left * right * top * bottom;
			if visible > highest {
				highest = visible;
			}
		}
	}
    highest
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        let part1 = solve_part_1(DATA);
        assert_eq!(part1, 21);
    }

    #[test]
    fn test_part_2() {
        let part2 = solve_part_2(DATA);
        assert_eq!(part2, 8);
    }
}