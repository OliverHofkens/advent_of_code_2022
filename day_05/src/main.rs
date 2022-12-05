use std::env;
use std::fs;

type Stacks = Vec<Vec<char>>;
type Move = (usize, usize, usize);

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();

    let (mut setup_1, moves) = parse_input(&inp);
    let mut setup_2 = setup_1.clone();

    for mv in &moves {
        // Puzzle 1:
        for _ in 0..mv.0 {
            let item = setup_1[mv.1 - 1].pop().unwrap();
            setup_1[mv.2 - 1].push(item);
        }

        // Puzzle 2:
        let start_idx = setup_2[mv.1 - 1].len() - mv.0;
        let mut moving: Vec<char> = setup_2[mv.1 - 1].drain(start_idx..).collect();
        setup_2[mv.2 - 1].append(&mut moving);
    }

    println!("Puzzle 1: {:?}", top_crates(setup_1));
    println!("Puzzle 2: {:?}", top_crates(setup_2));
}

fn parse_input(inp: &str) -> (Stacks, Vec<Move>) {
    let mut inp_parts = inp.split("\n\n");

    let mut stacks = Vec::with_capacity(9);
    let inp_setup = inp_parts.next().unwrap();

    // Iterate over the lines from lowest to highest.
    let mut setup_lines = inp_setup.lines().rev();
    // The last line tells us how many stacks there are
    let stack_numbers = setup_lines.nth(0).unwrap();
    for _ in stack_numbers.split_whitespace() {
        stacks.push(Vec::with_capacity(8));
    }

    for line in setup_lines {
        // Each column of the stack is 3 chars wide ([A]).
        // Columns are seperated by a single space.
        let cols = line.chars().skip(1).step_by(4);
        for (i, payload) in cols.enumerate() {
            match payload.is_alphabetic() {
                true => stacks[i].push(payload),
                false => continue,
            };
        }
    }

    let moves = inp_parts
        .next()
        .unwrap()
        .lines()
        .map(|m| {
            let nums: Vec<_> = m
                .split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            (nums[0], nums[1], nums[2])
        })
        .collect();

    (stacks, moves)
}

fn top_crates(setup: Stacks) -> String {
    setup
        .iter()
        .map(|s| s.iter().last().unwrap())
        .collect::<String>()
}
