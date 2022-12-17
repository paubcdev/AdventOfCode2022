use std::process::exit;

static ROCKS: &str =
"1111

0100
1110
0100

0010
0010
1110

1000
1000
1000
1000

1100
1100
";
const ROUNDS: usize = 2022;

pub fn main1() {
    let input = include_str!("../../inputs/day17.txt");
    let gas_jets = parse_input(input);
    let rocks = parse_rocks();
    //println!("{:?}", rocks);
    let mut cave = Cave::new(rocks, gas_jets);
    let height = cave.simulate();
    println!("{}", height);
}

struct Cave {
    rocks: Vec<Vec<u8>>,
    gas_jets: Vec<i8>,
    jet_idx: usize,
    cave: Vec<u8>,
}

impl Cave {
    fn new(rocks: Vec<Vec<u8>>, gas_jets: Vec<i8>) -> Self {
        Cave {
            rocks,
            gas_jets,
            jet_idx: 0,
            cave: Vec::new(),
        }
    }

    fn simulate(&mut self) -> usize {
        for i in 0..ROUNDS {
            let mut rock = self.rocks[i % self.rocks.len()].clone();
            let mut height = self.cave.len() + 3;
            loop {
                self.apply_jet(&mut rock, height);
                if self.move_down(&mut rock, height) {
                    // Rock has settled
                    break;
                }
                height -= 1
            }
        }
        self.cave.len()
    }

    fn apply_jet(&mut self, rock: &mut Vec<u8>, height: usize) {
        let dir = self.gas_jets[self.jet_idx];
        self.jet_idx = (self.jet_idx + 1) % self.gas_jets.len();
        let mut rock_cp = rock.clone();
        match dir {
            -1 => if rock_cp.iter().all(|slice| (slice & 1u8 << 6) == 0) {
                for slice in rock_cp.iter_mut() {
                    *slice <<= 1
                }
            },
            1 => if rock_cp.iter().all(|slice| (slice & 1u8) == 0) {
                for slice in rock_cp.iter_mut() {
                    *slice >>= 1
                }
            },
            _ => panic!(),
        }
        if !self.collides(&rock_cp, height) {
            *rock = rock_cp
        }
    }

    fn move_down(&mut self, rock: &Vec<u8>, height: usize) -> bool {
        if height == 0 || self.collides(rock, height - 1) {
            self.settle(rock, height);
            return true
        }
        false
    }

    fn settle(&mut self, rock: &Vec<u8>, height: usize) {
        for i in 0..rock.len() {
            let h = height + i;
            if h < self.cave.len() {
                self.cave[h] |= rock[i]
            } else {
                self.cave.push(rock[i])
            }
        }
    }

    // Whether this rock would collide with another rock
    fn collides(&self, rock: &[u8], height: usize) -> bool {
        if height >= self.cave.len() {
            return false
        }
        for (i, slice) in self.cave[height..].iter().enumerate() {
            if i >= rock.len() {
                break
            }
            if slice & rock[i] != 0 {
                return true
            }
        }
        false
    }
}

fn parse_input(input: &str) -> Vec<i8> {
    input.trim_end_matches('\n').as_bytes().iter()
        .map(|b| match b {
            b'<' => -1,
            b'>' => 1,
            c => {
                eprintln!("Unrecognized character {c} in input");
                exit(2);
            },
        }).collect()
}

fn parse_rocks() -> Vec<Vec<u8>> {
    let mut rocks_out = Vec::new();
    for rock in ROCKS.split("\n\n") {
        let mut new_rock: Vec<u8> = rock.lines()
            .map(|l| u8::from_str_radix(l, 2).unwrap() << 1)
            .collect();
        new_rock.reverse();
        rocks_out.push(new_rock);
    }
    rocks_out
}