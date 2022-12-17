use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs;

type TunnelMap = Vec<Vec<usize>>;
type Valves = Vec<Valve>;

#[derive(Debug)]
struct Valve {
    idx: usize,
    label: String,
    flow_rate: u64,
}

#[derive(Eq, Hash, PartialEq)]
struct State {
    current_valve: usize,
    mins_left: u64,
    valve_state: u64,
}

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();
    let (valves, tunnels) = parse_input(&inp);

    // Calculate the shortest distance between valves:
    let dist = floyd_warshall(&tunnels);

    // Now that we know all shortest distances, we have no reason to visit a
    // valve without flow rate:
    let start_valve = valves.iter().find(|v| v.label == "AA").unwrap();
    let worthy_valves: Vec<&Valve> = valves.iter().filter(|v| v.flow_rate > 0).collect();

    let mut cache: HashMap<State, u64> = HashMap::new();
    let res = check_options(&mut cache, &dist, &worthy_valves, &start_valve, 30, 0);
    println!("Part 1: {}", res);
}

fn floyd_warshall(map: &TunnelMap) -> Vec<Vec<u64>> {
    let n = map.len();
    let mut dist = vec![vec![999; n]; n];

    // Distance to itself is 0
    for i in 0..n {
        dist[i][i] = 0;
    }

    // Distance to directly connected tunnels is 1
    for (i, conns) in map.iter().enumerate() {
        for c in conns {
            dist[i][*c] = 1;
        }
    }

    // Find minimum distances for all pairs
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                dist[j][k] = cmp::min(dist[j][k], dist[j][i] + dist[i][k]);
            }
        }
    }
    dist
}

fn check_options(
    cache: &mut HashMap<State, u64>,
    dist: &Vec<Vec<u64>>,
    valves: &Vec<&Valve>,
    current_valve: &Valve,
    mins_left: u64,
    valve_state: u64,
) -> u64 {
    if mins_left <= 1 {
        return 0;
    }
    let state = State {
        current_valve: current_valve.idx,
        mins_left,
        valve_state,
    };
    if let Some(cache_hit) = cache.get(&state) {
        return *cache_hit;
    }

    let mut new_mins_left = mins_left;
    let mut new_valve_state = valve_state;
    let mut flow = 0;

    let possible_next: Vec<_> = valves
        .iter()
        .filter(|v| v.idx != current_valve.idx)
        .collect();

    // What's the best we can do if we don't open this valve now:
    let best_if_skipped = possible_next
        .iter()
        .filter_map(|v| {
            let _dist = dist[current_valve.idx][v.idx];
            match _dist + 1 < new_mins_left {
                true => Some(check_options(
                    cache,
                    dist,
                    valves,
                    v,
                    new_mins_left - _dist,
                    new_valve_state,
                )),
                false => None,
            }
        })
        .max()
        .unwrap_or(0);

    // If the valve is not opened and worth opening, do so now:
    if current_valve.flow_rate > 0 && (valve_state & (1 << current_valve.idx) == 0) {
        new_valve_state = valve_state | (1 << current_valve.idx);
        new_mins_left = mins_left - 1;
        flow = new_mins_left * current_valve.flow_rate;
    }

    let best = possible_next
        .iter()
        .filter_map(|v| {
            let _dist = dist[current_valve.idx][v.idx];
            match _dist + 1 < new_mins_left {
                true => Some(check_options(
                    cache,
                    dist,
                    valves,
                    v,
                    new_mins_left - _dist,
                    new_valve_state,
                )),
                false => None,
            }
        })
        .max()
        .unwrap_or(0);

    let res = cmp::max(best_if_skipped, best + flow);
    cache.insert(state, res);
    res
}

fn parse_input(inp: &str) -> (Valves, TunnelMap) {
    let mut label_to_idx: HashMap<String, usize> = HashMap::new();
    let mut valves = Vec::new();
    let mut tunnels = Vec::new();

    for (i, line) in inp.lines().enumerate() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let name = parts[1];
        let flow_rate = parts[4]
            .split("=")
            .nth(1)
            .unwrap()
            .trim_end_matches(';')
            .parse::<u64>()
            .unwrap();

        valves.push(Valve {
            idx: i,
            label: name.to_string(),
            flow_rate,
        });
        label_to_idx.insert(name.to_string(), i);
    }

    for line in inp.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let conns: Vec<usize> = parts[9..]
            .iter()
            .map(|c| label_to_idx[c.trim_end_matches(',')])
            .collect();
        tunnels.push(conns);
    }

    (valves, tunnels)
}
