use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::fs;

fn main() {
    let inp = get_input_contents();
    let (map, start, end, period) = parse_input(&inp);

    let p1 = best_path(&map, (start.0, start.1, 0), end, period).unwrap();
    println!("Part 1: {p1}");

    let back_to_start = best_path(&map, (end.0, end.1, p1 as i32), start, period).unwrap();
    let second_time_to_end = best_path(
        &map,
        (start.0, start.1, (p1 + back_to_start) as i32),
        end,
        period,
    )
    .unwrap();
    let p2 = p1 + back_to_start + second_time_to_end;
    println!("Part 2: {p2}");
}

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

type Pos = (i32, i32);
// Brilliant idea shamelessly stolen off of Reddit
type PosAtTime = (i32, i32, i32);

fn parse_input(input: &str) -> (HashSet<PosAtTime>, Pos, Pos, i32) {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let period = lcm(width - 2, height - 2);

    let mut map: HashSet<(i32, i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| (x as i32, y as i32, c))
        })
        .flat_map(|(x, y, c)| match c {
            '#' => (0..period).map(|z| (x, y, z)).collect::<Vec<_>>(),
            '<' => (0..period)
                .map(|z| ((x - 1 - z).rem_euclid(width - 2) + 1, y, z))
                .collect::<Vec<_>>(),
            '>' => (0..period)
                .map(|z| ((x - 1 + z).rem_euclid(width - 2) + 1, y, z))
                .collect::<Vec<_>>(),
            '^' => (0..period)
                .map(|z| (x, (y - 1 - z).rem_euclid(height - 2) + 1, z))
                .collect::<Vec<_>>(),
            'v' => (0..period)
                .map(|z| (x, (y - 1 + z).rem_euclid(height - 2) + 1, z))
                .collect::<Vec<_>>(),
            c => panic!("Unexpected char {c}"),
        })
        .collect();

    let start = (
        input
            .lines()
            .next()
            .unwrap()
            .chars()
            .position(|c| c == '.')
            .unwrap() as i32,
        0,
    );
    let end = (
        input
            .lines()
            .last()
            .unwrap()
            .chars()
            .position(|c| c == '.')
            .unwrap() as i32,
        height - 1,
    );

    // Prevent simply walking 'around' the map.
    for z in 0..period {
        map.insert((start.0, start.1 - 1, z));
        map.insert((end.0, end.1 + 1, z));
    }

    (map, start, end, period)
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b).abs() / gcd(a, b)
}

fn clear_neighbors(map: &HashSet<PosAtTime>, (x, y, z): PosAtTime, period: i32) -> Vec<PosAtTime> {
    let next_moment = (z + 1) % period;
    let candidates = [
        (x - 1, y, next_moment),
        (x + 1, y, next_moment),
        (x, y - 1, next_moment),
        (x, y + 1, next_moment),
        (x, y, next_moment),
    ];

    candidates
        .into_iter()
        .filter(|p| !map.contains(p))
        .collect()
}

fn best_path(map: &HashSet<PosAtTime>, start: PosAtTime, end: Pos, period: i32) -> Option<usize> {
    let mut distances = HashMap::new();
    let mut queue = BinaryHeap::new();

    distances.insert(start, 0);
    queue.push((Reverse(0), start));

    while let Some((Reverse(distance), position)) = queue.pop() {
        if (position.0, position.1) == end {
            return Some(distance);
        }

        for neighbor in clear_neighbors(map, position, period) {
            let neighbor_distance = distances.entry(neighbor).or_insert(usize::MAX);

            if *neighbor_distance > distance + 1 {
                *neighbor_distance = distance + 1;
                queue.push((Reverse(*neighbor_distance), neighbor));
            }
        }
    }

    None
}
