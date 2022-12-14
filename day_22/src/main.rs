use std::env;
use std::fs;

fn main() {
    let inp = get_input_contents();
    let (map, moves) = parse_input(&inp);

    let mut pos1 = Pos {
        x: map[0].iter().position(|c| *c == '.').unwrap(),
        y: 0,
        dir_x: 1,
        dir_y: 0,
    };
    let mut pos2 = pos1.clone();

    do_moves(&mut pos1, &map, &moves, false);
    let p1 = pos1.password();
    println!("Part 1: {p1}");

    do_moves(&mut pos2, &map, &moves, true);
    let p2 = pos2.password();
    println!("Part 2: {p2}");
}

#[derive(Debug)]
enum Move {
    TurnCW,
    TurnCCW,
    Forward(u64),
}
type Map = Vec<Vec<char>>;
type Moves = Vec<Move>;

#[derive(Debug, Clone)]
struct Pos {
    x: usize,
    y: usize,
    dir_x: i32,
    dir_y: i32,
}

impl Pos {
    fn password(&self) -> usize {
        let facing_idx = match (self.dir_x, self.dir_y) {
            (1, 0) => 0,
            (0, 1) => 1,
            (-1, 0) => 2,
            (0, -1) => 3,
            x => panic!("Unexpected facing {:?}", x),
        };
        1000 * (self.y + 1) + 4 * (self.x + 1) + facing_idx
    }
}

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn parse_input(inp: &str) -> (Map, Moves) {
    let (map_str, moves_str) = inp.split_once("\n\n").unwrap();

    let mut map: Vec<Vec<char>> = map_str.lines().map(|l| l.chars().collect()).collect();

    let mut moves = Vec::new();
    let mut buf: String = String::with_capacity(4);
    let mut chars = moves_str.chars();
    while let Some(c) = chars.next() {
        match c {
            'R' => {
                if buf.len() > 0 {
                    moves.push(Move::Forward(buf.parse::<u64>().unwrap()));
                    buf.clear();
                }
                moves.push(Move::TurnCW);
            }
            'L' => {
                if buf.len() > 0 {
                    moves.push(Move::Forward(buf.parse::<u64>().unwrap()));
                    buf.clear();
                }
                moves.push(Move::TurnCCW);
            }
            n if n.is_digit(10) => buf.push(n),
            _ => (),
        }
    }
    if buf.len() > 0 {
        moves.push(Move::Forward(buf.parse::<u64>().unwrap()));
    }

    // Ensure all rows of the map are of equal length
    let width = map.iter().map(|r| r.len()).max().unwrap();
    for r in &mut map {
        if r.len() < width {
            r.resize(width, ' ');
        }
    }

    (map, moves)
}

fn do_moves(pos: &mut Pos, map: &Map, moves: &Moves, cube_map: bool) {
    let mut drawmap = map.clone();

    for mv in moves {
        match mv {
            Move::TurnCW => {
                let prev_y = pos.dir_y;
                pos.dir_y = pos.dir_x;
                pos.dir_x = -1 * prev_y;
            }
            Move::TurnCCW => {
                let prev_x = pos.dir_x;
                pos.dir_x = pos.dir_y;
                pos.dir_y = -1 * prev_x;
            }
            Move::Forward(n) => match cube_map {
                true => step_cube(*n, pos, map, &mut drawmap),
                false if pos.dir_y == 0 => step_horizontal(*n, pos, map),
                false => step_vertical(*n, pos, map),
            },
        }
    }

    for (i, line) in drawmap.iter().enumerate() {
        print!("{i:>3} ");
        line.iter().for_each(|c| print!("{c}"));
        print!("\n");
    }
}

fn step_horizontal(n: u64, pos: &mut Pos, map: &Map) {
    let row = &map[pos.y];
    let width = row.len() as i32;

    for _ in 0..n {
        let check_pos = pos.x as i32 + pos.dir_x;

        let next_pos = match check_pos {
            x if pos.dir_x < 0 && (x < 0 || row[x as usize] == ' ') => {
                let offset = row.iter().rev().position(|c| *c != ' ').unwrap();
                width as usize - offset - 1
            }
            x if pos.dir_x > 0 && (x > width - 1 || row[x as usize] == ' ') => {
                row.iter().position(|c| *c != ' ').unwrap()
            }
            x => x as usize,
        };
        match row[next_pos] {
            '.' => pos.x = next_pos,
            '#' => return,
            c => panic!("Unexpected block {c} at {next_pos},{}", pos.y),
        }
    }
}

fn step_vertical(n: u64, pos: &mut Pos, map: &Map) {
    let height = map.len() as i32;

    for _ in 0..n {
        let check_pos = pos.y as i32 + pos.dir_y;

        let next_pos = match check_pos {
            y if pos.dir_y < 0 && (y < 0 || map[y as usize][pos.x] == ' ') => {
                let offset = map
                    .iter()
                    .rev()
                    .map(|r| r[pos.x])
                    .take_while(|c| *c == ' ')
                    .count();
                height as usize - offset - 1
            }
            y if pos.dir_y > 0 && (y > height - 1 || map[y as usize][pos.x] == ' ') => {
                let offset = map
                    .iter()
                    .map(|r| r[pos.x])
                    .take_while(|c| *c == ' ')
                    .count();
                offset
            }
            y => y as usize,
        };
        match map[next_pos][pos.x] {
            '.' => pos.y = next_pos,
            '#' => return,
            c => panic!("Unexpected block {c} at {},{next_pos}", pos.x),
        }
    }
}

fn step_cube(n: u64, pos: &mut Pos, map: &Map, drawmap: &mut Map) {
    // No time for a clean, generic solution. Fck it we ball

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    for _ in 0..n {
        let mut next_x = pos.x;
        let mut next_y = pos.y;
        let mut next_dirx = pos.dir_x;
        let mut next_diry = pos.dir_y;

        // Horizontal movement
        if pos.dir_x != 0 {
            let check_pos = pos.x as i32 + pos.dir_x;

            match check_pos {
                // Move left and wrap around:
                x if pos.dir_x < 0 && (x < 0 || map[pos.y][x as usize] == ' ') => {
                    match pos.y {
                        // Face 0 to face 3
                        y if y < 50 => {
                            next_dirx = 1;
                            next_diry = 0;
                            next_x = 0;
                            next_y = 149 - y;
                        }
                        // Face 2 to face 3
                        y if y < 100 => {
                            next_dirx = 0;
                            next_diry = 1;
                            next_x = y - 50;
                            next_y = 100;
                        }
                        // Face 3 to face 0
                        y if y < 150 => {
                            next_dirx = 1;
                            next_diry = 0;
                            next_x = 50;
                            next_y = 149 - y;
                        }
                        // Face 5 to face 0
                        y => {
                            next_dirx = 0;
                            next_diry = 1;
                            next_x = y - 100;
                            next_y = 0;
                        }
                    }
                }
                // Move right and wrap around:
                x if pos.dir_x > 0 && (x > width - 1 || map[pos.y][x as usize] == ' ') => {
                    match pos.y {
                        // Face 1 to face 4
                        y if y < 50 => {
                            next_dirx = -1;
                            next_diry = 0;
                            next_x = 99;
                            next_y = 149 - y;
                        }
                        // Face 2 to face 1
                        y if y < 100 => {
                            next_dirx = 0;
                            next_diry = -1;
                            next_x = y + 50;
                            next_y = 49;
                        }
                        // Face 4 to face 1
                        y if y < 150 => {
                            next_dirx = -1;
                            next_diry = 0;
                            next_x = 149;
                            next_y = 149 - y;
                        }
                        // Face 5 to face 4
                        y => {
                            next_dirx = 0;
                            next_diry = -1;
                            next_x = y - 100;
                            next_y = 149;
                        }
                    }
                }
                // No wrapping:
                x => {
                    next_x = x as usize;
                }
            };
        }
        // Vertical movement
        else {
            let check_pos = pos.y as i32 + pos.dir_y;

            match check_pos {
                // Move up and wrap around:
                y if pos.dir_y < 0 && (y < 0 || map[y as usize][pos.x] == ' ') => match pos.x {
                    // Face 3 to face 2
                    x if x < 50 => {
                        next_dirx = 1;
                        next_diry = 0;
                        next_x = 50;
                        next_y = 50 + x;
                    }
                    // Face 0 to face 5
                    x if x < 100 => {
                        next_dirx = 1;
                        next_diry = 0;
                        next_x = 0;
                        next_y = 100 + x;
                    }
                    // Face 1 to face 5
                    x => {
                        next_dirx = 0;
                        next_diry = -1;
                        next_x = x - 100;
                        next_y = 199;
                    }
                },
                // Move down and wrap around:
                y if pos.dir_y > 0 && (y > height - 1 || map[y as usize][pos.x] == ' ') => {
                    match pos.x {
                        // Face 5 to face 1
                        x if x < 50 => {
                            next_dirx = 0;
                            next_diry = 1;
                            next_x = x + 100;
                            next_y = 0;
                        }
                        // Face 4 to face 5
                        x if x < 100 => {
                            next_dirx = -1;
                            next_diry = 0;
                            next_x = 49;
                            next_y = x + 100;
                        }
                        // Face 1 to face 2
                        x => {
                            next_dirx = -1;
                            next_diry = 0;
                            next_x = 99;
                            next_y = x - 50;
                        }
                    }
                }
                // No wrapping:
                y => {
                    next_y = y as usize;
                }
            };
        }

        match map[next_y][next_x] {
            '.' => {
                pos.dir_x = next_dirx;
                pos.dir_y = next_diry;
                pos.x = next_x;
                pos.y = next_y;

                drawmap[pos.y][pos.x] = match (pos.dir_x, pos.dir_y) {
                    (1, 0) => '>',
                    (0, 1) => 'v',
                    (-1, 0) => '<',
                    (0, -1) => '^',
                    _ => 'o',
                };
            }
            '#' => return,
            c => panic!("Unexpected block {c} at {next_x},{next_y}"),
        }
    }
}
