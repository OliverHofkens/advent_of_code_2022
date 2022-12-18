use std::collections::HashSet;
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
}

fn parse_input(inp: &str) -> HashSet<Cube> {
    inp.lines()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect::<Vec<_>>())
        .map(|parts| (parts[0], parts[1], parts[2]))
        .collect()
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
