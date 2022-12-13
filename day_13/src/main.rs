use std::cmp::Ordering;
use std::env;
use std::fs;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();
    let pairs = parse_input(&inp);

    let p1: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, pkts)| match pkts[0] < pkts[1] {
            true => Some(i + 1),
            false => None,
        })
        .sum();
    println!("Part 1: {}", p1);

    let dividers: Vec<Packet> = ["[[2]]", "[[6]]"].iter().map(|p| (*p).into()).collect();
    let mut inp_p2: Vec<Packet> = inp
        .lines()
        .filter(|l| l.len() > 0)
        .map(|x| x.into())
        .collect();
    inp_p2.extend_from_slice(&dividers[..]);
    inp_p2.sort_unstable();

    let p2: usize = inp_p2
        .iter()
        .enumerate()
        .filter_map(|(i, pkt)| match dividers.contains(pkt) {
            true => Some(i + 1),
            false => None,
        })
        .product();
    println!("Part 2: {}", p2);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Val {
    Int(u32),
    List(Vec<Val>),
}

fn _cmp_lists(a: &[Val], b: &[Val]) -> Ordering {
    for (l, r) in a.iter().zip(b.iter()) {
        match l.cmp(r) {
            Ordering::Equal => continue,
            o => return o,
        };
    }
    // No clear ordering from items in list, compare lengths:
    a.len().cmp(&b.len())
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> Ordering {
        let res = match (self, other) {
            (Val::Int(l), Val::Int(r)) => l.cmp(r),
            (Val::List(l), Val::List(r)) => _cmp_lists(&l, &r),
            (Val::List(l), Val::Int(r)) => _cmp_lists(&l, &[Val::Int(*r)]),
            (Val::Int(l), Val::List(r)) => _cmp_lists(&[Val::Int(*l)], &r),
        };
        res
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    data: Vec<Val>,
}

impl From<&str> for Packet {
    fn from(item: &str) -> Self {
        match str_to_packet_val(item) {
            Val::List(v) => Packet { data: v },
            Val::Int(_) => panic!("Expected list, found int"),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        _cmp_lists(&self.data, &other.data)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(inp: &str) -> Vec<Vec<Packet>> {
    inp.split("\n\n")
        .map(|pair| pair.lines().map(|line| line.into()).collect())
        .collect()
}

fn str_to_packet_val(repr: &str) -> Val {
    let mut res: Option<Val> = None;
    let mut stack = Vec::new();
    let mut buf = String::new();

    for c in repr.chars() {
        match c {
            '[' => stack.push(Vec::new()),
            ']' => {
                if !buf.is_empty() {
                    stack
                        .last_mut()
                        .unwrap()
                        .push(Val::Int(buf.parse::<u32>().unwrap()));
                    buf.clear();
                }
                let done = Val::List(stack.pop().unwrap());
                match stack.last_mut() {
                    Some(s) => s.push(done),
                    None => res = Some(done),
                }
            }
            ',' => {
                if !buf.is_empty() {
                    stack
                        .last_mut()
                        .unwrap()
                        .push(Val::Int(buf.parse::<u32>().unwrap()));
                    buf.clear();
                }
            }
            x if x.is_digit(10) => buf.push(x),
            x => panic!("Unexpected char {}", x),
        }
    }
    res.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        assert_eq!(Val::Int(1) == Val::Int(1), true);
        assert_eq!(Val::Int(2) > Val::Int(1), true);
        assert_eq!(Val::Int(1) < Val::Int(2), true);

        assert_eq!(Val::List(vec![]) == Val::List(vec![]), true);
        assert_eq!(Val::List(vec![]) < Val::List(vec![Val::Int(1)]), true);
        assert_eq!(Val::List(vec![Val::Int(1)]) > Val::List(vec![]), true);

        assert_eq!(
            Val::List(vec![Val::Int(1)]) == Val::List(vec![Val::Int(1)]),
            true
        );
        assert_eq!(
            Val::List(vec![Val::Int(1)]) < Val::List(vec![Val::Int(2)]),
            true
        );
        assert_eq!(
            Val::List(vec![Val::Int(2)]) > Val::List(vec![Val::Int(1)]),
            true
        );

        assert_eq!(
            Val::List(vec![Val::List(vec![]), Val::Int(1)])
                < Val::List(vec![Val::List(vec![]), Val::Int(2)]),
            true
        );
        assert_eq!(
            Val::List(vec![Val::List(vec![Val::Int(0)]), Val::Int(1)])
                < Val::List(vec![Val::List(vec![Val::Int(0)]), Val::Int(2)]),
            true
        );

        assert_eq!(
            Val::Int(1) < Val::List(vec![Val::Int(1), Val::Int(2)]),
            true
        );
        assert_eq!(
            Val::List(vec![Val::List(vec![]), Val::Int(7)])
                < Val::List(vec![Val::List(vec![Val::Int(3)])]),
            true
        )
    }
}
