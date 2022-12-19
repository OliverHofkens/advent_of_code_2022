#![feature(iter_array_chunks)]

use std::cmp::max;
use std::collections::HashMap;
use std::env;
use std::fs;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();
    let blueprints = parse_input(&inp);
    println!("{:?}", blueprints);

    let initial_state = State {
        mins_left: 24,
        resources: [0; 4],
        bots: [1, 0, 0, 0],
    };

    let quality_sum: u64 = blueprints
        .iter()
        .map(|b| {
            let best = max_geodes(&mut HashMap::new(), &b, &initial_state);
            println!("Blueprint {}: Max geodes: {}", b.id, best);
            b.id * best
        })
        .sum();
    println!("Part 1: {}", quality_sum);
}

// Resource indices:
const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

// Cost of (ore, clay, obsidian)
type Cost = [u64; 3];

#[derive(Debug)]
struct Blueprint {
    id: u64,
    costs: [Cost; 4],
    // OPTIMIZATION: We don't need to build bots of a type, if we're already
    // producing enough of this resource to build the most expensive bot
    // each minute.
    max_prod_needed: [u64; 4],
}

impl From<&str> for Blueprint {
    fn from(item: &str) -> Self {
        let (part_id, part_costs) = item.split_once(":").unwrap();
        let id = part_id
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let mut costs = [[0; 3]; 4];
        let mut max_prod_needed = [0, 0, 0, u64::MAX];

        for cost_desc in part_costs.split(".").filter(|s| s.len() > 1) {
            let parts: Vec<&str> = cost_desc.split_whitespace().collect();
            let cost_idx = match parts[1] {
                "ore" => ORE,
                "clay" => CLAY,
                "obsidian" => OBSIDIAN,
                "geode" => GEODE,
                x => panic!("Unexpected robot type {}", x),
            };

            for res_cost in parts.iter().skip(3).array_chunks::<3>() {
                let cost = res_cost[1].parse::<u64>().unwrap();
                let res_idx = match res_cost[2] {
                    &"ore" => ORE,
                    &"clay" => CLAY,
                    &"obsidian" => OBSIDIAN,
                    x => panic!("Unexpected resource cost {}", x),
                };
                costs[cost_idx][res_idx] = cost;
                max_prod_needed[res_idx] = max(max_prod_needed[res_idx], cost);
            }
        }

        Self {
            id,
            costs,
            max_prod_needed,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    mins_left: usize,
    resources: [u64; 4],
    bots: [u64; 4],
}

fn parse_input(inp: &str) -> Vec<Blueprint> {
    inp.lines().map(|l| l.into()).collect()
}

fn max_geodes(cache: &mut HashMap<State, u64>, blueprint: &Blueprint, state: &State) -> u64 {
    if state.mins_left == 0 {
        return state.resources[GEODE];
    }

    if let Some(hit) = cache.get(&state) {
        return *hit;
    }

    let mut next_state = state.clone();
    next_state.mins_left -= 1;
    // Harvest resources:
    for res in ORE..=GEODE {
        next_state.resources[res] += state.bots[res];
    }

    // Try our different options:
    let mut outcomes: Vec<u64> = Vec::new();
    // 1: Do nothing:
    outcomes.push(max_geodes(cache, blueprint, &next_state));

    // 2: Attempt to build some bots:
    // OPTIMIZATION: Don't build a bot if it can't harvest anyway:
    if next_state.mins_left > 0 {
        for bot in ORE..=GEODE {
            // OPTIMIZATION: Don't overproduce unneccessary bots:
            if state.bots[bot] >= blueprint.max_prod_needed[bot] {
                continue;
            }

            let cost = blueprint.costs[bot];
            if cost.iter().zip(state.resources).all(|(c, r)| *c <= r) {
                let mut _state = next_state.clone();
                _state.bots[bot] += 1;
                for res in ORE..=OBSIDIAN {
                    _state.resources[res] -= cost[res];
                }
                outcomes.push(max_geodes(cache, blueprint, &_state));
            }
        }
    }

    let res = *outcomes.iter().max().unwrap();
    cache.insert(state.clone(), res);
    res
}
