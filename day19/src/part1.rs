use priority_queue::DoublePriorityQueue;
use std::cmp::Ordering;

type SimType = u32;
type CostVal = SimType;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Cost {
    ore: CostVal,
    clay: CostVal,
    obs: CostVal,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct SimState {
    ore: SimType,
    clay: SimType,
    obs: SimType,
    geo: SimType,
    rob_ore: SimType,
    rob_clay: SimType,
    rob_obs: SimType,
    rob_geo: SimType,
}

impl SimState {
    #[inline]
    fn priority(&self) -> SimType {
        self.geo.wrapping_shl(24)
            + self.rob_geo.wrapping_shl(16)
            + self.obs
            + self.rob_obs
            + self.rob_clay
    }
}

impl Ord for SimState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl PartialOrd for SimState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn sim_blueprint(init_state: SimState, minutes: usize, costs: [Cost; 4]) -> SimState {
    let queue_size = 2 << 11;
    let mut prev_states = DoublePriorityQueue::<SimState, SimType>::with_capacity(queue_size);
    prev_states.push(init_state, init_state.priority());
    let mut new_states = DoublePriorityQueue::<SimState, SimType>::with_capacity(queue_size);
    for _min in 1..minutes {
        #[cfg(debug_assertions)]
        println!("min: {}, {}", _min, prev_states.len());
        for (state, _priority) in prev_states.iter() {
            for i in 0..=costs.len() {
                // continue sim as if robot i was purchased
                let mut state = *state;
                if i < costs.len() {
                    let cost = costs[i];
                    if state.ore >= cost.ore && state.clay >= cost.clay && state.obs >= cost.obs {
                        state.ore -= cost.ore;
                        state.clay -= cost.clay;
                        state.obs -= cost.obs;
                        state.ore += state.rob_ore;
                        state.clay += state.rob_clay;
                        state.obs += state.rob_obs;
                        state.geo += state.rob_geo;
                        match i {
                            0 => {
                                state.rob_ore += 1;
                            }
                            1 => {
                                state.rob_clay += 1;
                            }
                            2 => {
                                state.rob_obs += 1;
                            }
                            3 => {
                                state.rob_geo += 1;
                            }
                            _ => unreachable!(),
                        }
                        new_states.push(state, state.priority());
                    }
                } else {
                    state.ore += state.rob_ore;
                    state.clay += state.rob_clay;
                    state.obs += state.rob_obs;
                    state.geo += state.rob_geo;
                    new_states.push(state, state.priority());
                }
                if new_states.len() > queue_size - 10 {
                    for _ in 0..10 {
                        new_states.pop_min();
                    }
                }
            }
        }
        prev_states.clear();
        (new_states, prev_states) = (prev_states, new_states);
        #[cfg(debug_assertions)]
        for (state, priority) in prev_states.iter() {
            if _min == 1
                && *state
                    == (SimState {
                        ore: 1,
                        clay: 0,
                        obs: 0,
                        geo: 0,
                        rob_ore: 1,
                        rob_clay: 0,
                        rob_obs: 0,
                        rob_geo: 0,
                    })
            {
                println!("    {}: {:?}", priority, state);
            }
            if _min == 4
                && *state
                    == (SimState {
                        ore: 4,
                        clay: 0,
                        obs: 0,
                        geo: 0,
                        rob_ore: 1,
                        rob_clay: 0,
                        rob_obs: 0,
                        rob_geo: 0,
                    })
            {
                println!("    {}: {:?}", priority, state);
            }
            if _min == 8
                && *state
                    == (SimState {
                        ore: 3,
                        clay: 1,
                        obs: 0,
                        geo: 0,
                        rob_ore: 2,
                        rob_clay: 2,
                        rob_obs: 0,
                        rob_geo: 0,
                    })
            {
                println!("    {}: {:?}", priority, state);
            }
            if _min == 16
                && *state
                    == (SimState {
                        ore: 3,
                        clay: 14,
                        obs: 2,
                        geo: 0,
                        rob_ore: 2,
                        rob_clay: 7,
                        rob_obs: 2,
                        rob_geo: 0,
                    })
            {
                println!("    {}: {:?}", priority, state);
            }
            if _min == 20
                && *state
                    == (SimState {
                        ore: 3,
                        clay: 14,
                        obs: 7,
                        geo: 0,
                        rob_ore: 2,
                        rob_clay: 7,
                        rob_obs: 4,
                        rob_geo: 1,
                    })
            {
                println!("    {}: {:?}", priority, state);
            }
            if _min == 24
                && *state
                    == (SimState {
                        ore: 2,
                        clay: 28,
                        obs: 5,
                        geo: 7,
                        rob_ore: 2,
                        rob_clay: 7,
                        rob_obs: 5,
                        rob_geo: 4,
                    })
            {
                println!("    {}: {:?}", priority, state);
            }
        }
    }
    #[cfg(debug_assertions)]
    println!("min: {}, {}", minutes, prev_states.len());
    prev_states
        .into_sorted_iter()
        .rev()
        .map(|(mut state, _priority)| {
            state.ore += state.rob_ore;
            state.clay += state.rob_clay;
            state.obs += state.rob_obs;
            state.geo += state.rob_geo;
            #[cfg(debug_assertions)]
            if state
                == (SimState {
                    ore: 6,
                    clay: 41,
                    obs: 8,
                    geo: 9,
                    rob_ore: 1,
                    rob_clay: 4,
                    rob_obs: 2,
                    rob_geo: 2,
                })
            {
                println!("    {:?}", state);
            }
            state
        })
        .max_by(|a, b| a.geo.cmp(&b.geo))
        .unwrap()
}

pub fn part_1_solver(input: &str) -> u32 {
    let blueprints = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut words = l.split(' ');
            let id = words
                .nth(1)
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse::<CostVal>()
                .unwrap();
            let ore_cost = words.nth(4).unwrap().parse::<CostVal>().unwrap();
            let clay_cost = words.nth(5).unwrap().parse::<CostVal>().unwrap();
            let obs_ore_cost = words.nth(5).unwrap().parse::<CostVal>().unwrap();
            let obs_clay_cost = words.nth(2).unwrap().parse::<CostVal>().unwrap();
            let geode_ore_cost = words.nth(5).unwrap().parse::<CostVal>().unwrap();
            let geode_obs_cost = words.nth(2).unwrap().parse::<CostVal>().unwrap();

            (
                id,
                [
                    Cost {
                        ore: ore_cost,
                        clay: 0,
                        obs: 0,
                    },
                    Cost {
                        ore: clay_cost,
                        clay: 0,
                        obs: 0,
                    },
                    Cost {
                        ore: obs_ore_cost,
                        clay: obs_clay_cost,
                        obs: 0,
                    },
                    Cost {
                        ore: geode_ore_cost,
                        clay: 0,
                        obs: geode_obs_cost,
                    },
                ],
            )
        });

    let state = SimState {
        // time: 0,
        ore: 0,
        clay: 0,
        obs: 0,
        geo: 0,
        rob_ore: 1,
        rob_clay: 0,
        rob_obs: 0,
        rob_geo: 0,
    };
    let results = blueprints
        .map(|(i, costs)| {
            let state = sim_blueprint(state, 24, costs);
            #[cfg(debug_assertions)]
            {
                println!("{:?}", state);
                println!("blueprint {} had at most {} geodes", i, state.geo);
            }
            (i, state.geo)
        })
        .collect::<Vec<_>>();
    let result = results.iter().map(|(i, geo)| i * geo).sum::<SimType>();
    result

}

pub fn main1() {
    let input =  include_str!("../../inputs/day19.txt");
    print!("Part 1: {} ", part_1_solver(input));
}
