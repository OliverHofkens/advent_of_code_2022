use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

fn main() {
    let inp = get_input_contents();
    let mut positions = parse_input(&inp);

    let (bottom_left, top_right) = bounding_box(&positions);
    simulate(&mut positions, 10);
    draw_map(&positions);
    let p1 = empty_space(&positions);
    println!("Part 1: {p1}");
}

type Pos = (i64, i64);
enum Dir {
    N,
    E,
    S,
    W,
}

fn simulate(positions: &mut HashSet<Pos>, rounds: usize) {
    let mut dir_order = VecDeque::from([Dir::N, Dir::S, Dir::W, Dir::E]);

    // <proposed, from>
    let mut proposed: HashMap<Pos, Vec<Pos>> = HashMap::with_capacity(positions.len());

    for _rnd in 0..rounds {
        // Proposal phase:
        for pos in positions.iter() {
            let adj = neighbors(pos);
            let occupied: Vec<bool> = adj.iter().map(|p| positions.contains(p)).collect();

            // No elves around, stay where we are:
            if !occupied.iter().any(|o| *o) {
                continue;
            }

            for check_dir in &dir_order {
                let (free, new_pos) = match check_dir {
                    Dir::N => (!occupied[0..3].iter().any(|o| *o), (pos.0, pos.1 - 1)),
                    Dir::E => (!occupied[2..5].iter().any(|o| *o), (pos.0 + 1, pos.1)),
                    Dir::S => (!occupied[4..7].iter().any(|o| *o), (pos.0, pos.1 + 1)),
                    Dir::W => (
                        !occupied[6..].iter().any(|o| *o) && !occupied[0],
                        (pos.0 - 1, pos.1),
                    ),
                };
                if free {
                    proposed
                        .entry(new_pos)
                        .and_modify(|x| x.push(pos.clone()))
                        .or_insert_with(|| vec![pos.clone()]);
                    break;
                }
            }
        }

        // Move phase:
        for (to, from) in proposed.drain() {
            // Only move if exactly 1 elf wants to move there
            if from.len() == 1 {
                // Remove the old pos:
                positions.remove(&from[0]);
                // Add the new pos:
                positions.insert(to);
            }
        }

        // Rotate phase
        dir_order.rotate_left(1);
    }
}

fn neighbors(pos: &Pos) -> Vec<Pos> {
    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ]
    .iter()
    .map(|(dx, dy)| (pos.0 + dx, pos.1 + dy))
    .collect()
}

fn bounding_box(positions: &HashSet<Pos>) -> (Pos, Pos) {
    let min_x = positions.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = positions.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = positions.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = positions.iter().map(|(_, y)| *y).max().unwrap();
    ((min_x, min_y), (max_x, max_y))
}

fn empty_space(positions: &HashSet<Pos>) -> usize {
    let ((min_x, min_y), (max_x, max_y)) = bounding_box(positions);
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    width * height - positions.len()
}

fn draw_map(positions: &HashSet<Pos>) {
    let ((min_x, min_y), (max_x, max_y)) = bounding_box(positions);
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut map = vec![vec!['.'; width]; height];

    for (x, y) in positions {
        map[(y - min_y) as usize][(x - min_x) as usize] = '#'
    }

    for row in &map {
        row.iter().for_each(|c| print!("{c}"));
        print!("\n");
    }
}

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn parse_input(inp: &str) -> HashSet<Pos> {
    inp.lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .flatten()
        .collect()
}
