use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

type Cube = (i8, i8, i8);

fn main() {
    let inp = get_input_contents();
    let cubes = parse_input(&inp);

    let p1: usize = cubes
        .iter()
        .map(|d| boundary(*d).iter().filter(|b| !cubes.contains(b)).count())
        .sum();
    println!("Part 1: {}", p1);

    let (lower_bound, upper_bound) = bounding_box(&cubes);
    let p2 = flood(&cubes, lower_bound, upper_bound);
    println!("Part 2: {}", p2);
}

fn parse_input(inp: &str) -> HashSet<Cube> {
    inp.lines()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect::<Vec<_>>())
        .map(|parts| (parts[0], parts[1], parts[2]))
        .collect()
}

fn bounding_box(cubes: &HashSet<Cube>) -> (Cube, Cube) {
    let min_x = cubes.iter().map(|c| c.0).min().unwrap() - 2;
    let max_x = cubes.iter().map(|c| c.0).max().unwrap() + 2;
    let min_y = cubes.iter().map(|c| c.1).min().unwrap() - 2;
    let max_y = cubes.iter().map(|c| c.1).max().unwrap() + 2;
    let min_z = cubes.iter().map(|c| c.1).min().unwrap() - 2;
    let max_z = cubes.iter().map(|c| c.1).max().unwrap() + 2;

    ((min_x, min_y, min_z), (max_x, max_y, max_z))
}

fn boundary(c: Cube) -> [Cube; 6] {
    [
        (c.0 + 1, c.1, c.2),
        (c.0 - 1, c.1, c.2),
        (c.0, c.1 + 1, c.2),
        (c.0, c.1 - 1, c.2),
        (c.0, c.1, c.2 + 1),
        (c.0, c.1, c.2 - 1),
    ]
}

fn flood(drop: &HashSet<Cube>, start: Cube, end: Cube) -> usize {
    let mut visited: HashSet<Cube> = HashSet::new();
    let mut queue = VecDeque::new();
    let mut hits_on_drop = 0;

    queue.push_back(start);
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        if !(visited.contains(&curr)
            || curr.0 > end.0
            || curr.0 < start.0
            || curr.1 > end.1
            || curr.1 < start.1
            || curr.2 > end.2
            || curr.2 < start.2)
        {
            visited.insert(curr);
            boundary(curr).into_iter().for_each(|n| {
                if drop.contains(&n) {
                    hits_on_drop += 1;
                } else {
                    queue.push_back(n);
                }
            });
        }
    }
    hits_on_drop
}
