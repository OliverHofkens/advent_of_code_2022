use std::collections::VecDeque;
use std::env;
use std::fs;
use std::rc::Rc;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<i64>,
    op: Rc<dyn Fn(i64) -> i64>,
    reducer: Rc<dyn Fn(i64) -> i64>,
    test_modul: i64,
    target_if_true: usize,
    target_if_false: usize,
    n_inspects: u64,
}

impl Monkey {
    fn inspect(&mut self) -> Option<i64> {
        let mut worry = self.items.pop_front()?;
        worry = (self.op)(worry);
        worry = (self.reducer)(worry);
        self.n_inspects += 1;
        Some(worry)
    }
}

fn main() {
    let inp = get_input_contents();

    let mut monkeys = parse_input_monkeys(&inp);
    let mut monkeys_2 = monkeys.clone();

    let common_factor: i64 = monkeys_2.iter().map(|m| m.test_modul).product();
    let reducer = Rc::new(move |x| x % common_factor);
    for m in &mut monkeys_2 {
        m.reducer = reducer.clone();
    }

    println!("Puzzle 1: {}", solve(&mut monkeys, 20));
    println!("Puzzle 2: {}", solve(&mut monkeys_2, 10_000));
}

fn solve(monkeys: &mut Vec<Monkey>, rounds: usize) -> u64 {
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            loop {
                let monk = &mut monkeys[i];
                let item = match monk.inspect() {
                    Some(i) => i,
                    None => break,
                };
                let target = match item % monk.test_modul {
                    0 => monk.target_if_true,
                    _ => monk.target_if_false,
                };
                monkeys[target].items.push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|m| m.n_inspects);
    let monkey_business: u64 = monkeys.iter().rev().take(2).map(|m| m.n_inspects).product();
    monkey_business
}

fn parse_input_monkeys(inp: &str) -> Vec<Monkey> {
    inp.split("\n\n").map(|p| parse_monkey(p)).collect()
}

fn parse_monkey(inp: &str) -> Monkey {
    let mut lines = inp.lines().skip(1);
    let items: VecDeque<i64> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();
    let op_desc: Vec<_> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect();
    let op: Rc<dyn Fn(i64) -> i64> = match &op_desc[..] {
        &["new", "=", "old", "*", "old"] => Rc::new(|v| v * v),
        &["new", "=", "old", "*", x] => {
            let mult = x.parse::<i64>().unwrap();
            Rc::new(move |v| v * mult)
        }
        &["new", "=", "old", "+", x] => {
            let add = x.parse::<i64>().unwrap();
            Rc::new(move |v| v + add)
        }
        e => panic!("Failed to parse {:?}", e),
    };
    let test_modul = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let target_if_true = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let target_if_false = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    Monkey {
        items,
        op,
        reducer: Rc::new(|x| x / 3),
        test_modul,
        target_if_true,
        target_if_false,
        n_inspects: 0,
    }
}
