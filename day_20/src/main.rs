use std::collections::VecDeque;
use std::env;
use std::fs;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();
    let encrypted = parse_input(&inp);

    let decrypted = mix_decrypt(&encrypted, 1);
    let p1 = sum_coords(&decrypted);
    println!("Part 1: {}", p1);

    const DECRYPTION_KEY: i64 = 811589153;
    let encrypted: VecDeque<i64> = encrypted.iter().map(|x| x * DECRYPTION_KEY).collect();
    let decrypted = mix_decrypt(&encrypted, 10);
    let p2 = sum_coords(&decrypted);
    println!("Part 2: {}", p2);
}

fn parse_input(inp: &str) -> VecDeque<i64> {
    inp.lines().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn mix_decrypt(instr: &VecDeque<i64>, rounds: usize) -> VecDeque<i64> {
    let mut mix: VecDeque<(usize, i64)> = instr.clone().iter().copied().enumerate().collect();
    let size = mix.len();

    for _ in 0..rounds {
        for (idx, instr) in instr.iter().enumerate() {
            if *instr == 0 {
                continue;
            }
            let actual_idx = mix.iter().position(|(i, _)| *i == idx).unwrap();

            mix.rotate_left(actual_idx);
            let (old_idx, elem) = mix.pop_front().unwrap();
            let positions = elem.abs() as usize % (size - 1);
            match elem > 0 {
                true => mix.rotate_left(positions),
                false => mix.rotate_right(positions),
            };
            mix.push_front((old_idx, elem));
        }
    }

    mix.iter().map(|(_, elem)| *elem).collect()
}

fn sum_coords(decrypted: &VecDeque<i64>) -> i64 {
    let pos_of_zero = decrypted.iter().position(|x| *x == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| decrypted[((pos_of_zero + i) % decrypted.len())])
        .sum()
}
