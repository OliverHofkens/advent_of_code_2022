use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::env;
use std::fs;

type Pos = (usize, usize);

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();
    let (height_map, start, end) = parse_input(&inp);
    let adj_map = height_map_to_adj_map(&height_map);

    println!("Puzzle 1: {}", dijkstra(&adj_map, start, end).unwrap());
    println!(
        "Puzzle 2: {}",
        adj_map
            .keys()
            .filter_map(|&(i, j)| match height_map[j][i] {
                0 => dijkstra(&adj_map, (i, j), end),
                _ => None,
            })
            .min()
            .unwrap()
    );
}

fn parse_input(inp: &str) -> (Vec<Vec<i8>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut height_map: Vec<Vec<i8>> = Vec::new();

    for (j, line) in inp.lines().enumerate() {
        let mut map_line = Vec::new();

        for (i, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    map_line.push(0);
                    start = (i, j);
                }
                'E' => {
                    map_line.push(25);
                    end = (i, j);
                }
                x => map_line.push((x as u32 - 'a' as u32) as i8),
            }
        }

        height_map.push(map_line);
    }

    (height_map, start, end)
}

fn height_map_to_adj_map(height_map: &Vec<Vec<i8>>) -> BTreeMap<Pos, Vec<Pos>> {
    let width = height_map[0].len();
    let height = height_map.len();

    let mut res = BTreeMap::new();
    for i in 0..width {
        for j in 0..height {
            let reachable_neighbors = [(-1, 0), (1, 0), (0, 1), (0, -1)]
                .iter()
                .map(|(di, dj)| (di + i as isize, dj + j as isize))
                .filter_map(|(ia, ja)| {
                    if (0..width as isize).contains(&ia) && (0..height as isize).contains(&ja) {
                        Some((ia as usize, ja as usize))
                    } else {
                        None
                    }
                })
                .filter(|&(ia, ja)| &height_map[ja][ia] <= &(height_map[j][i] + 1))
                .collect();

            res.insert((i, j), reachable_neighbors);
        }
    }
    res
}

fn dijkstra(graph: &BTreeMap<Pos, Vec<Pos>>, start: Pos, end: Pos) -> Option<usize> {
    // Current best cost for each point
    let mut dist: BTreeMap<Pos, usize> = graph.keys().map(|pos| (*pos, usize::MAX)).collect();
    // Priority queue for which points to visit
    let mut verts = BinaryHeap::new();

    // Cost at starting point is known and zero
    dist.insert(start, 0);
    // Start by visiting the start point
    // In order to get a min-heap, we need to wrap items in `Reverse`,
    // see: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#min-heap
    verts.push(Reverse((0, start.0, start.1)));

    while let Some(Reverse((cost, i, j))) = verts.pop() {
        // Reached the end!
        if (i, j) == end {
            return Some(cost);
        }

        // Do we already have a better path to this position?
        if cost > dist[&(i, j)] {
            continue;
        }

        // For each reachable point, check if this is a better path.
        for neighbor in &graph[&(i, j)] {
            let new_cost = cost + 1;
            if new_cost < dist[&(neighbor.0, neighbor.1)] {
                verts.push(Reverse((new_cost, neighbor.0, neighbor.1)));
                *dist.get_mut(&(neighbor.0, neighbor.1)).unwrap() = new_cost;
            }
        }
    }

    None
}
