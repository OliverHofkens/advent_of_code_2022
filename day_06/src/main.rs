use std::env;
use std::fs;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();

    let start_idx = get_marker_idx(&inp, 4);
    let msg_idx = get_marker_idx(&inp, 14);
    println!("Puzzle 1: {}", start_idx);
    println!("Puzzle 2: {}", msg_idx);
}

fn get_marker_idx(msg: &str, packet_size: usize) -> usize {
    let inp_chars: Vec<char> = msg.chars().collect();

    let (packet_start_idx, _packet) = inp_chars
        .windows(packet_size)
        .enumerate()
        .find(|(_, seq)| is_unique(seq))
        .unwrap();
    packet_start_idx + packet_size
}

fn is_unique(slice: &[char]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_marker_idx() {
        let msg = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(get_marker_idx(msg, 4), 7);
        assert_eq!(get_marker_idx(msg, 14), 19);
    }
}
