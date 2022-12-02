use std::env;
use std::fs;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();

    let mut score_1 = 0u32;
    let mut score_2 = 0u32;

    for round in inp.lines() {
        let mut chars = round.chars();
        let opp_move = chars.nth(0).unwrap();

        let my_move_p1 = chars.nth(1).unwrap();
        score_1 += calc_score(my_move_p1, opp_move);

        let my_move_p2 = match my_move_p1 {
            'X' => match opp_move {
                'A' => 'Z',
                'B' => 'X',
                'C' => 'Y',
                _ => panic!(),
            },
            'Y' => match opp_move {
                'A' => 'X',
                'B' => 'Y',
                'C' => 'Z',
                _ => panic!(),
            },
            'Z' => match opp_move {
                'A' => 'Y',
                'B' => 'Z',
                'C' => 'X',
                _ => panic!(),
            },
            _ => panic!(),
        };
        score_2 += calc_score(my_move_p2, opp_move);
    }

    println!("Puzzle 1: {}", score_1);
    println!("Puzzle 2: {}", score_2);
}

fn calc_score(my_move: char, opp_move: char) -> u32 {
    match my_move {
        'X' => match opp_move {
            'A' => 4,
            'C' => 7,
            _ => 1,
        },
        'Y' => match opp_move {
            'A' => 8,
            'B' => 5,
            _ => 2,
        },
        'Z' => match opp_move {
            'B' => 9,
            'C' => 6,
            _ => 3,
        },
        _ => panic!(),
    }
}
