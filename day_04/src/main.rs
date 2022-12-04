use std::env;
use std::fs;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();

    let mut full_overlaps: u32 = 0;
    let mut partial_overlaps: u32 = 0;

    for pair in inp.lines() {
        let ranges: Vec<(u32, u32)> = pair.split(",").map(|x| parse_range(x)).collect();
        match is_full_overlap(ranges[0], ranges[1]) {
            true => full_overlaps += 1,
            _ => (),
        }
        match is_partial_overlap(ranges[0], ranges[1]) {
            true => partial_overlaps += 1,
            _ => (),
        }
    }
    println!("Puzzle 1: {}", full_overlaps);
    println!("Puzzle 2: {}", partial_overlaps);
}

fn parse_range(range: &str) -> (u32, u32) {
    let parts: Vec<u32> = range
        .split("-")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    (parts[0], parts[1])
}

fn is_full_overlap(a: (u32, u32), b: (u32, u32)) -> bool {
    if (a.0 <= b.0) && (a.1 >= b.1) {
        return true;
    }
    if (b.0 <= a.0) && (b.1 >= a.1) {
        return true;
    }
    false
}

fn is_partial_overlap(a: (u32, u32), b: (u32, u32)) -> bool {
    if (a.1 >= b.0) && (a.0 <= b.1) {
        return true;
    }
    false
}
