use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cube {
    position: (i64, i64, i64),
    sides: [(i64, i64, i64); 6],
}

impl Cube {
    #[must_use]
    pub const fn new(position @ (x, y, z): (i64, i64, i64)) -> Self {
        let x = x * 2;
        let y = y * 2;
        let z = z * 2;

        Self {
            position,
            sides: Self::sides((x, y, z)),
        }
    }

    #[must_use]
    pub const fn sides((x, y, z): (i64, i64, i64)) -> [(i64, i64, i64); 6] {
        [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ]
    }
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tt = s.split(',').map(|s| s.parse::<i64>().unwrap());

        let x = tt.next().unwrap();
        let y = tt.next().unwrap();
        let z = tt.next().unwrap();

        Ok(Self::new((x, y, z)))
    }
}

fn fill(
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
    z_range: RangeInclusive<i64>,
    cubes: &HashSet<Cube>,
) -> HashSet<(i64, i64, i64)> {
    let start = (*x_range.start(), *y_range.start(), *z_range.start());
    let cube_positions = cubes.iter().map(|c| c.position).collect::<HashSet<_>>();
    let mut water = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);

    while let Some(position @ (x, y, z)) = queue.pop_front() {
        if x_range.contains(&x)
            && y_range.contains(&y)
            && z_range.contains(&z)
            && !water.contains(&position)
            && !cube_positions.contains(&position)
        {
            water.insert(position);
            queue.extend(Cube::sides(position));
        }
    }

    water
}

pub fn part_2_solver(input: &str) -> usize {
    let cubes = input
        .lines()
        .map(|l| l.parse::<Cube>().unwrap())
        .collect::<HashSet<Cube>>();

    let mut x_min = i64::MAX;
    let mut x_max = i64::MIN;
    let mut y_min = i64::MAX;
    let mut y_max = i64::MIN;
    let mut z_min = i64::MAX;
    let mut z_max = i64::MIN;

    for cube in &cubes {
        let (x, y, z) = cube.position;

        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
        z_min = z_min.min(z);
        z_max = z_max.max(z);
    }

    let wet_sides = fill(
        x_min - 1..=x_max + 1,
        y_min - 1..=y_max + 1,
        z_min - 1..=z_max + 1,
        &cubes,
    )
    .into_iter()
    .map(Cube::new)
    .collect::<HashSet<_>>()
    .iter()
    .flat_map(|c| c.sides)
    .collect::<HashSet<_>>();

    let mut n = 0;
    let sides = cubes.iter().flat_map(|c| c.sides).collect::<Vec<_>>();
    let mut shared = HashSet::new();

    for cube in &cubes {
        if cube.sides.iter().any(|s| wet_sides.contains(s)) {
            for side in &cube.sides {
                if wet_sides.contains(side) && !shared.contains(side) {
                    match sides.iter().filter(|s| s == &side).count() {
                        1 => n += 1,
                        2 => {
                            n += 1;

                            shared.insert(*side);
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    n
}

pub fn main_part_2() {
    let input = include_str!("../../inputs/day18.txt");
    println!("Part 2: {}", part_2_solver(input.trim()));
}