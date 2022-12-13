use itertools::Itertools;
use serde_json::Value;
use std::cmp::*;

fn comp(a: &Value, b: &Value) -> Ordering {
  match (a,b) {
    (Value::Number(x), Value::Number(y)) => x.as_u64().unwrap().cmp(&y.as_u64().unwrap()),
    (Value::Array(a), Value::Array(b)) => {
      for i in 0..max(a.len(), b.len()) {
        match (a.get(i), b.get(i)) {
          (None, _) => return Ordering::Less,
          (_, None) => return Ordering::Greater,
          (Some(x), Some(y)) => match comp(x,y) {
            Ordering::Equal => {},
            c => return c,
          }
        }
      }
      Ordering::Equal
    },
    (Value::Array(_), Value::Number(_)) => comp(a, &Value::Array(vec![b.clone()])),
    (Value::Number(_), Value::Array(_)) => comp(&Value::Array(vec![a.clone()]), b),
    _ => unreachable!(),
  }
}

fn part_1_solver(input: &str) -> usize {
    let signals = input.lines()
    .filter(|l| !l.is_empty())
    .map(|l| serde_json::from_str::<Value>(l).unwrap())
    .collect::<Vec<_>>();
    let part1: usize = signals.iter()
    .tuples()
    .positions(|(a,b)| comp(a,b) != Ordering::Greater)
    .map(|i| i + 1).sum();
    part1
}

fn part_2_solver(input: &str) -> usize {
    let mut signals = input.lines()
    .filter(|l| !l.is_empty())
    .map(|l| serde_json::from_str::<Value>(l).unwrap())
    .collect::<Vec<_>>();
    let beacons = [
    serde_json::from_str::<Value>("[[2]]").unwrap(),
    serde_json::from_str::<Value>("[[6]]").unwrap(),];
    signals.extend(beacons.iter().cloned());
    signals.sort_by(comp);
    let part2: usize = signals.iter().positions(|b| beacons.contains(b)).map(|i| i + 1).product();
    part2
}

fn main() {
    let input = include_str!("../../inputs/day13.txt");
    println!("Part 1: {}", part_1_solver(input));
    println!("Part 2: {}", part_2_solver(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_solver(DATA), 13);
    }
    
    #[test]
    fn test_part_2() {
        assert_eq!(part_2_solver(DATA), 140);
    }
} 