use std::env;
use std::fs;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();

    let supply_per_elf: Vec<Option<u32>> = inp.lines().map(|i| i.parse::<u32>().ok()).collect();
    let mut sums: Vec<u32> = supply_per_elf
        .split(|i| i.is_none())
        .map(|c| c.iter().map(|val| val.unwrap()).sum())
        .collect();
    sums.sort();

    println!("Puzzle 1: {}", sums[sums.len() - 1]);
    println!("Puzzle 2: {}", sums.iter().rev().take(3).sum::<u32>());
}
