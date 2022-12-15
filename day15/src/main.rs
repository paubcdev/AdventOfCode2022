use itertools::Itertools;

fn part_1_solver(input: &str) -> i64 {
    let beacons = input.lines().map(|l|
        l.split(|c: char| !c.is_digit(10) && c != '-')
        .filter_map(|w| w.parse::<i64>().ok())
        .collect_tuple()
        .map(|(x,y,dx,dy)| (x, y, (x - dx).abs() + (y - dy).abs()))
        .unwrap()
    ).collect::<Vec<_>>();
    let compressed = beacons.iter()
        .map(|&(x,y,d)| (x, d - (2000000 - y).abs()))
        .filter(|&(_,left)| left >= 0)
        .flat_map(|(x,left)| [(x - left, true), (x + left + 1, false)])
        .sorted().collect::<Vec<_>>();
    let (mut result, mut inside) = (-1, 1);
    for ((prev, _), &(x, start)) in compressed.iter().tuple_windows() {
        if inside > 0 { result += x - prev }
        inside += if start {1} else {-1};
    }
    result
}

fn part_2_solver(input: &str) -> i64 {
    let beacons = input.lines().map(|l|
        l.split(|c: char| !c.is_digit(10) && c != '-')
        .filter_map(|w| w.parse::<i64>().ok())
        .collect_tuple()
        .map(|(x,y,dx,dy)| (x, y, (x - dx).abs() + (y - dy).abs()))
        .unwrap()
    ).collect::<Vec<_>>();
    for &(x,y,d) in &beacons {
        for (dir_x, dir_y) in [(-1,-1), (-1,1), (1,-1), (1,1)] {
            for dist in 0..d {
                let bx = x + dir_x * dist;
                let by = y + dir_y * (d + 1 - dist);
                if bx < 0 || by < 0 || bx >= 4000000 || by >= 4000000 {
                    break;
                }
                if beacons.iter().all(|&(x,y,d)| (bx - x).abs() + (by - y).abs() >= d) {
                    let res = (bx * 4000000) + by;
                    return res
                }
            }
        }
    }
    panic!("Something went wrong")
}

fn main() {
    let input = include_str!("../../inputs/day15.txt");
    
    let part1 = part_1_solver(input);
    let part2 = part_2_solver(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}