use std::collections::HashSet;
use std::env;
use std::fs;

type Pos = (i64, i64);

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();

    let mut knots = vec![(0, 0); 10];
    let mut visited_p1: HashSet<Pos> = HashSet::new();
    let mut visited_p2: HashSet<Pos> = HashSet::new();

    for line in inp.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let dir = parts[0];
        let dist = parts[1].parse::<i64>().unwrap();

        let (dx, dy) = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            unk => panic!("Unknown direction {}", unk),
        };

        for _ in 0..dist {
            knots[0].0 += dx;
            knots[0].1 += dy;

            for idx in 1..knots.len() {
                let head = knots[idx - 1];
                let mut tail = &mut knots[idx];
                follow(&head, &mut tail);
            }
            visited_p1.insert(knots[1]);
            visited_p2.insert(knots[9]);
        }
        //draw(&knots);
    }

    println!("Puzzle 1: {}", visited_p1.len());
    println!("Puzzle 2: {}", visited_p2.len());
}

fn follow(head: &Pos, tail: &mut Pos) {
    match (head.0 - tail.0, head.1 - tail.1) {
        // Explicitly do nothing, so the catch-all can report
        // on the truly weird cases.
        (x, y) if x >= -1 && x <= 1 && y >= -1 && y <= 1 => (),
        // Right
        (x, y) if x > 1 && y == 0 => tail.0 += 1,
        // Left
        (x, y) if x < -1 && y == 0 => tail.0 -= 1,
        // Up
        (x, y) if x == 0 && y > 1 => tail.1 += 1,
        // Down
        (x, y) if x == 0 && y < -1 => tail.1 -= 1,
        // Up+Right
        (x, y) if (x > 1 && y >= 1) || (x == 1 && y > 1) => {
            tail.0 += 1;
            tail.1 += 1;
        }
        // Down+Right
        (x, y) if (x > 1 && y <= -1) || (x == 1 && y < -1) => {
            tail.0 += 1;
            tail.1 -= 1
        }
        // Up+Left
        (x, y) if (x < -1 && y >= 1) || (x == -1 && y > 1) => {
            tail.0 -= 1;
            tail.1 += 1;
        }
        // Down+Left
        (x, y) if (x < -1 && y <= -1) || (x == -1 && y < -1) => {
            tail.0 -= 1;
            tail.1 -= 1;
        }
        _ => panic!("Not sure how to follow from {:?} to {:?}", head, tail),
    }
}

fn draw(knots: &Vec<Pos>) {
    let bottom_left = (-20, -20);

    let mut canvas = vec![vec!['.'; 50]; 50];

    for (i, pos) in knots.iter().enumerate() {
        let x_idx = pos.0 - bottom_left.0;
        let y_idx = pos.1 - bottom_left.1;
        canvas[50 - y_idx as usize][x_idx as usize] =
            char::from_digit(i.try_into().unwrap(), 10).unwrap();
    }

    for line in canvas {
        for char in line {
            print!("{} ", char);
        }
        print!("\n");
    }
}
