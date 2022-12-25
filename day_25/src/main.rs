use std::env;
use std::fs;

fn main() {
    let inp = get_input_contents();

    let sum_dec: i64 = inp.lines().map(|l| i64::from(SNAFU(l.to_string()))).sum();
    let sum_snafu: SNAFU = sum_dec.into();
    println!("Part 1: {} = {}", sum_dec, sum_snafu.0);
}

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

struct SNAFU(String);

impl From<SNAFU> for i64 {
    fn from(item: SNAFU) -> Self {
        item.0
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let digit = match c {
                    x if x.is_digit(3) => x.to_digit(10).unwrap() as i64,
                    '-' => -1,
                    '=' => -2,
                    x => panic!("Invalid SNAFU digit {x}"),
                };
                5i64.pow(i as u32) * digit
            })
            .sum()
    }
}

impl From<i64> for SNAFU {
    fn from(mut item: i64) -> Self {
        let mut res = Vec::new();
        while item > 0 {
            match item % 5 {
                x @ (0 | 1 | 2) => res.push(char::from_digit(x as u32, 10).unwrap()),
                3 => {
                    res.push('=');
                    item += 2;
                }
                4 => {
                    res.push('-');
                    item += 1
                }
                _ => panic!(),
            };

            item /= 5;
        }

        SNAFU(res.into_iter().rev().collect())
    }
}
