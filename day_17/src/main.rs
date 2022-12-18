use std::collections::{hash_map::Entry, HashMap};
use std::env;
use std::fs;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();
    let jets = parse_input(&inp);

    let mut tower = Vec::new();

    const P1_N_ROCKS: usize = 2022;
    let mut wind_idx = 0;
    for shape in SHAPES.into_iter().cycle().take(P1_N_ROCKS) {
        wind_idx = simulate_rock(&mut tower, &jets, wind_idx, shape);
    }
    println!("Part 1: {}", tower.len());

    tower.clear();
    wind_idx = 0;

    const P2_N_ROCKS: usize = 1_000_000_000_000;
    const PATTERN_BLOCK_SIZE: usize = 16;
    let mut seen_states = HashMap::with_capacity(1_024);
    let mut cycle_height = 0;
    let mut n = 0;
    while n < P2_N_ROCKS {
        let shape_idx = n % SHAPES.len();
        let shape = SHAPES[shape_idx];

        wind_idx = simulate_rock(&mut tower, &jets, wind_idx, shape);
        n += 1;

        if tower.len() < PATTERN_BLOCK_SIZE {
            continue;
        }

        // Check if we've seen this pattern before:
        let block = u128::from_ne_bytes(
            tower[tower.len() - PATTERN_BLOCK_SIZE..]
                .try_into()
                .unwrap(),
        );
        let state = (block, shape_idx, wind_idx);

        match seen_states.entry(state) {
            Entry::Occupied(e) => {
                let (old_n, old_height) = e.get();
                let num_rocks_in_cycle = n - old_n;
                let num_cycles = (P2_N_ROCKS - n) / num_rocks_in_cycle;
                n += num_rocks_in_cycle * num_cycles;
                cycle_height += num_cycles * (tower.len() - old_height);
                seen_states.clear();
            }
            Entry::Vacant(e) => {
                e.insert((n, tower.len()));
            }
        }
    }

    println!("Part 2: {}", tower.len() + cycle_height);
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

// This genius move comes from Reddit user Gix:
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Shape(u32);

const SHAPES: [Shape; 5] = [
    // Shapes start with LEFT edge 2 spaces from the wall (so 7 - 2 => 5)
    Shape(0b00000000_00000000_00000000_00011110),
    Shape(0b00000000_00001000_00011100_00001000),
    Shape(0b00000000_00000100_00000100_00011100),
    Shape(0b00010000_00010000_00010000_00010000),
    Shape(0b00000000_00000000_00011000_00011000),
];
const LEFT_EDGE: Shape = Shape(0b01000000_01000000_01000000_01000000);
const RIGHT_EDGE: Shape = Shape(0b0000001_00000001_00000001_00000001);

impl Shape {
    const fn intersects(&self, mask: u32) -> bool {
        self.0 & mask != 0
    }

    fn blow(&mut self, direction: Dir, tower_mask: u32) {
        let new_pos = match direction {
            Dir::Left => {
                if self.0 & LEFT_EDGE.0 == 0 {
                    self.0 << 1
                } else {
                    return;
                }
            }
            Dir::Right => {
                if self.0 & RIGHT_EDGE.0 == 0 {
                    self.0 >> 1
                } else {
                    return;
                }
            }
        };

        if new_pos & tower_mask == 0 {
            self.0 = new_pos;
        }
    }

    fn as_lines(self) -> impl Iterator<Item = u8> {
        self.0.to_le_bytes().into_iter().take_while(|b| *b != 0)
    }
}

fn tower_slice_mask(tower: &[u8], height: usize) -> u32 {
    // Returns the tower mask at the given height.
    if height >= tower.len() {
        0
    } else {
        tower[height..]
            .iter()
            .take(4)
            .rev()
            .fold(0u32, |acc, b| (acc << 8) | *b as u32)
    }
}

fn simulate_rock(
    tower: &mut Vec<u8>,
    wind: &[Dir],
    mut wind_idx: usize,
    mut shape: Shape,
) -> usize {
    let mut height = tower.len() + 3;

    loop {
        let wind_dir = wind[wind_idx];
        wind_idx += 1;
        if wind_idx == wind.len() {
            wind_idx = 0;
        }

        let current_mask = tower_slice_mask(tower, height);

        shape.blow(wind_dir, current_mask);

        if height > tower.len() {
            height -= 1;
        } else if height == 0 || shape.intersects(tower_slice_mask(tower, height - 1)) {
            for byte in shape.as_lines() {
                if height < tower.len() {
                    tower[height] |= byte;
                } else {
                    tower.push(byte);
                }
                height += 1;
            }
            return wind_idx;
        } else {
            height -= 1;
        }
    }
}

fn parse_input(inp: &str) -> Vec<Dir> {
    inp.chars()
        .filter_map(|c| match c {
            '<' => Some(Dir::Left),
            '>' => Some(Dir::Right),
            _ => None,
        })
        .collect()
}
