#[derive(Copy, Clone, Debug)]
enum Cell {
    Air,
    Rock,
    Sand,
}

fn main() {
    let input: &str = include_str!("../../inputs/day14.txt");
    
    println!("Part 1: {}", part_1_solver(input));
    println!("Part 2: {}", part_2_solver(input));
}

fn part_1_solver(input: &str) -> i32 {

	let mut cave = [[Cell::Air; 600]; 600];

	let mut lowest = 0;

	let lines: Vec<_> = input.lines().collect();

	for line in lines {
		let instructions: Vec<_> = line.split(" -> ").collect();
		for i in 0..instructions.len()-1 {
			let pair1: Vec<usize> = instructions[i].split(',').map(|x| x.parse().unwrap()).collect();
			let pair2: Vec<usize> = instructions[i+1].split(',').map(|x| x.parse().unwrap()).collect();
			for j in pair1[0].min(pair2[0])..=pair1[0].max(pair2[0]) {
				for k in pair1[1].min(pair2[1])..=pair1[1].max(pair2[1]) {
					cave[j][k] = Cell::Rock;
				}
			}
			if pair1[1] > lowest {
				lowest = pair1[1];
			}
			if pair2[1] > lowest {
				lowest = pair2[1];
			}
		}
	}

	let mut count = 0;

	'outer: loop {
		let mut x = 500;
		let mut y = 0;
		loop {
			if y > lowest {
				break 'outer;				
			}
			if let Cell::Air = cave[x][y+1]	{
				y += 1;
				continue;
			} else if let Cell::Air = cave[x-1][y+1] {
				x -= 1;
				y += 1;
				continue;
			} else if let Cell::Air = cave[x+1][y+1] {
				x += 1;
				y += 1;
				continue;
			}
			cave[x][y] = Cell::Sand;	
			count += 1;
			break;
		}
	}
    count
}

fn part_2_solver(input: &str) -> i32 {
    let mut cave = [[Cell::Air; 900]; 900];

	let mut lowest = 0;

	let lines: Vec<_> = input.lines().collect();

	for line in lines {
		let instructions: Vec<_> = line.split(" -> ").collect();
		for i in 0..instructions.len()-1 {
			let pair1: Vec<usize> = instructions[i].split(',').map(|x| x.parse().unwrap()).collect();
			let pair2: Vec<usize> = instructions[i+1].split(',').map(|x| x.parse().unwrap()).collect();
			for j in pair1[0].min(pair2[0])..=pair1[0].max(pair2[0]) {
				for k in pair1[1].min(pair2[1])..=pair1[1].max(pair2[1]) {
					cave[j][k] = Cell::Rock;
				}
			}
			if pair1[1] > lowest {
				lowest = pair1[1];
			}
			if pair2[1] > lowest {
				lowest = pair2[1];
			}
		}
	}

	for i in 0..cave.len() {
		cave[i][lowest + 2] = Cell::Rock;
	}

	let mut count = 0;

	'outer: loop {
		let mut x = 500;
		let mut y = 0;
		loop {
			if let Cell::Air = cave[x][y+1]	{
				y += 1;
				continue;
			} else if let Cell::Air = cave[x-1][y+1] {
				x -= 1;
				y += 1;
				continue;
			} else if let Cell::Air = cave[x+1][y+1] {
				x += 1;
				y += 1;
				continue;
			} else if let Cell::Sand = cave[x][y] {
				if y == 0 {
					break 'outer;
				}
			}
			cave[x][y] = Cell::Sand;	
			count += 1;
			break;
		}
	}
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_solver(DATA), 24);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_solver(DATA), 93);
    }
}