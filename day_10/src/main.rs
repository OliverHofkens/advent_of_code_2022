use std::env;
use std::fs;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

#[derive(Debug)]
enum Instr {
    Noop,
    AddX(i64),
}

fn main() {
    let inp = get_input_contents();

    let mut procedure: Vec<Instr> = Vec::new();
    for instr in inp.lines().rev() {
        let parts: Vec<_> = instr.split_whitespace().collect();
        match parts[0] {
            "noop" => procedure.push(Instr::Noop),
            "addx" => procedure.push(Instr::AddX(parts[1].parse::<i64>().unwrap())),
            _ => (),
        }
    }

    let mut x: i64 = 1;
    let mut cycle: i64 = 1;
    let mut signal_strength: i64 = 0;
    let mut stack: Option<Instr> = None;

    loop {
        if (cycle + 20) % 40 == 0 {
            signal_strength += cycle * x;
        }
        let pix = (cycle - 1) % 40;
        match pix >= x - 1 && pix <= x + 1 {
            true => print!("#"),
            false => print!("."),
        }
        if cycle % 40 == 0 {
            print!("\n");
        }

        match stack {
            Some(Instr::AddX(add)) => {
                x += add;
                stack = None;
            }
            _ => match procedure.pop() {
                val @ Some(Instr::AddX(_)) => stack = val,
                Some(Instr::Noop) => (),
                None => break,
            },
        }

        cycle += 1;
    }

    println!("Puzzle 1: {}", signal_strength);
}
