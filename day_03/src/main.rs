use std::collections::HashSet;
use std::env;
use std::fs;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();

    let mut prio_1: u32 = 0;
    let mut prio_2: u32 = 0;

    let mut current_squad = [HashSet::new(), HashSet::new(), HashSet::new()];
    for (i, rucksack) in inp.lines().enumerate() {
        let midpoint: usize = rucksack.len() / 2;
        let mut chars = rucksack.chars();
        let comp_1: HashSet<char> = chars.by_ref().take(midpoint).collect();
        let comp_2: HashSet<char> = chars.by_ref().take(midpoint).collect();

        let common_item = comp_1.intersection(&comp_2).nth(0).unwrap();
        prio_1 += item_priority(common_item);

        let squad_idx = i % 3;
        current_squad[squad_idx] = &comp_1 | &comp_2;
        if squad_idx == 2 {
            let mut badge = &(&current_squad[0] & &current_squad[1]) & &current_squad[2];
            prio_2 += item_priority(&badge.drain().nth(0).unwrap());
        }
    }

    println!("Puzzle 1: {}", prio_1);
    println!("Puzzle 2: {}", prio_2);
}

fn item_priority(item: &char) -> u32 {
    let ascii_code = *item as u32;
    match item.is_lowercase() {
        true => ascii_code - 96,
        false => ascii_code - 64 + 26,
    }
}
