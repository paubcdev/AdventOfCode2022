
use std::process::exit;

use fxhash::FxHashMap;


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
const ROUNDS: usize = 1_000_000_000_000;

pub fn main2() {
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
    rock_idx: usize,
    gas_jets: Vec<i8>,
    jet_idx: usize,
    cave: Vec<u8>,
}

impl Cave {
    fn new(rocks: Vec<Vec<u8>>, gas_jets: Vec<i8>) -> Self {
        Cave {
            rocks,
            rock_idx: 0,
            gas_jets,
            jet_idx: 0,
            cave: Vec::new(),
        }
    }

    fn simulate(&mut self) -> usize {
        // Save the state after each rock to find a recurring pattern
        let mut states: FxHashMap<(Vec<u8>, usize, usize), (usize, usize)> = FxHashMap::default();
        let mut state = (Vec::new(), 0, 0);
        let mut idx = 0;
        for i in 0.. {
            self.rock_idx = i % self.rocks.len();
            self.add_rock();
            let cur_state = (self.get_state(), self.rock_idx, self.jet_idx);
            if states.contains_key(&cur_state) {
                idx = i;
                state = cur_state;
                break
            }
            states.insert(cur_state, (i, self.cave.len()));
        }
        let (prev_i, prev_height) = states[&state];
        let diff_h = self.cave.len() - prev_height;
        let diff_i = idx - prev_i;
        // Number of rocks in the partial cycle at the end
        let end = (ROUNDS - prev_i) % diff_i;
        for i in (idx + 1)..(idx + end) {
            self.rock_idx = i % self.rocks.len();
            self.add_rock();
        }
        let cycles = (ROUNDS - (prev_i + end)) / diff_i;
        // self.cave now contains the beginning, the first cycle and the end
        diff_h * (cycles - 1) + self.cave.len()
    }

    fn add_rock(&mut self) {
        let mut rock = self.rocks[self.rock_idx].clone();
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

    fn get_state(&self) -> Vec<u8> {
        let mut covered: u8 = 0;
        let mut top_shape: Vec<u8> = Vec::new();
        for slice in self.cave.iter().rev() {
            covered |= slice;
            top_shape.push(slice.clone());
            if covered == !(1 << 7) {
                break;
            }
        }
        top_shape
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
