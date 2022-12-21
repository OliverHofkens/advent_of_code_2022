use rand::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();
    let graph = parse_input(&inp);
    let p1 = solve(&graph);
    println!("Part 1: {}", p1);

    let p2 = goal_seeking(&graph);
    println!("Part 2: {}", p2);
}

#[derive(Debug, Clone)]
struct Task {
    lhs: String,
    rhs: String,
    op: char,
}

#[derive(Debug, Clone)]
enum Node {
    Literal(i64),
    Task(Task),
}

fn parse_input(inp: &str) -> HashMap<String, Node> {
    let mut res = HashMap::new();

    for line in inp.lines() {
        let (name, task) = line.split_once(": ").unwrap();
        let task_parts: Vec<_> = task.split_whitespace().collect();
        let node = match &task_parts[..] {
            &[x] => Node::Literal(x.parse::<i64>().unwrap()),
            &[lhs, op, rhs] => Node::Task(Task {
                lhs: lhs.to_string(),
                rhs: rhs.to_string(),
                op: op.chars().next().unwrap(),
            }),
            x => panic!("Failed to parse node {:?}", x),
        };
        res.insert(name.to_string(), node);
    }

    res
}

fn topological_sort(graph: &HashMap<String, Node>) -> Vec<String> {
    let mut unseen = (*graph).clone();
    let mut sorted = Vec::new();
    let mut no_deps: VecDeque<String> = graph
        .iter()
        .filter_map(|(key, val)| match val {
            Node::Literal(_) => {
                let (key_rem, _) = unseen.remove_entry(key).unwrap();
                Some(key_rem)
            }
            Node::Task(_) => None,
        })
        .collect();

    let mut to_remove: Vec<String> = Vec::new();
    while let Some(k) = no_deps.pop_front() {
        sorted.push(k.clone());

        // Find any nodes that no longer have unseen deps
        for (unseen_key, v) in &unseen {
            match v {
                Node::Task(Task { lhs, rhs, op: _ }) => {
                    if (lhs == &k && !unseen.contains_key(rhs))
                        || (rhs == &k && !unseen.contains_key(lhs))
                    {
                        to_remove.push(unseen_key.to_string());
                    }
                }
                _ => continue,
            }
        }
        for rem in to_remove.drain(..) {
            let (k, _) = unseen.remove_entry(&rem).unwrap();
            no_deps.push_back(k);
        }
    }

    if !unseen.is_empty() {
        panic!("Graph is cyclic!");
    }

    sorted
}

fn solve(graph: &HashMap<String, Node>) -> i64 {
    let sorted_keys = topological_sort(&graph);
    let mut vals: HashMap<String, i64> = HashMap::new();

    for key in sorted_keys {
        match graph.get(&key).unwrap() {
            Node::Literal(x) => vals.insert(key, *x),
            Node::Task(Task { lhs, op, rhs }) => {
                let lhs = vals.get(lhs).unwrap();
                let rhs = vals.get(rhs).unwrap();
                let val = match op {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    '/' => lhs / rhs,
                    '%' => lhs % rhs,
                    x => panic!("Unknown operator {}", x),
                };
                vals.insert(key, val)
            }
        };
    }

    vals["root"]
}

fn goal_seeking(graph: &HashMap<String, Node>) -> i64 {
    let mut sorted_keys = topological_sort(&graph);
    let mut vals: HashMap<String, i64> = HashMap::new();

    // We're gonna try to minimize the difference of the inputs in `root`:
    let mut graph = (*graph).clone();
    if let Node::Task(old_root) = graph.get("root").unwrap() {
        graph.insert(
            "root".to_string(),
            Node::Task(Task {
                lhs: old_root.lhs.clone(),
                rhs: old_root.rhs.clone(),
                op: '=',
            }),
        );
    }

    // Anything that doesn't depend on the `humn` node can be calculated up front:
    let drain_until = sorted_keys
        .iter()
        .position(|k| match graph.get(k).unwrap() {
            Node::Literal(_) => false,
            Node::Task(Task { lhs, rhs, op: _ }) => lhs == "humn" || rhs == "humn",
        })
        .unwrap();

    for key in sorted_keys.drain(0..drain_until) {
        println!("Pre-calculating {}", key);
        match graph.get(&key).unwrap() {
            Node::Literal(x) => vals.insert(key, *x),
            Node::Task(Task { lhs, op, rhs }) => {
                let lhs = vals.get(lhs).unwrap();
                let rhs = vals.get(rhs).unwrap();
                let val = match op {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    '/' => lhs / rhs,
                    '%' => lhs % rhs,
                    x => panic!("Unknown operator {}", x),
                };
                vals.insert(key, val)
            }
        };
    }

    // Probably overkill, but I just love differential evolution so much:
    differential_evolution(
        &mut move |x| _goal_seeking_run(x, &graph, &mut vals, &sorted_keys),
        7,
        0.9,
        100_000,
        1,
        1_000_000,
    )
    .unwrap()
}

fn _goal_seeking_run(
    guess: i64,
    graph: &HashMap<String, Node>,
    vals: &mut HashMap<String, i64>,
    task_order: &Vec<String>,
) -> i64 {
    vals.insert("humn".to_string(), guess);

    for key in task_order {
        match graph.get(key).unwrap() {
            Node::Task(Task { lhs, op, rhs }) => {
                let lhs = vals.get(lhs).unwrap();
                let rhs = vals.get(rhs).unwrap();
                let val = match op {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    '/' => lhs / rhs,
                    '%' => lhs % rhs,
                    // Express the diff as a cost we can minimize
                    '=' => (lhs - rhs).abs(),
                    x => panic!("Unknown operator {}", x),
                };
                vals.insert(key.to_string(), val)
            }
            _ => panic!("Unexpected literal node {} in goal seeking!", key),
        };
    }

    vals["root"]
}

#[derive(Debug, Clone)]
struct Agent {
    pos: f64,
    fitness: i64,
}

// We could make this generic, but that would required the `num` crate.
fn differential_evolution(
    func: &mut dyn FnMut(i64) -> i64,
    pop_size: usize,
    diff_weight: f64,
    max_iter: usize,
    lower_bound: i64,
    upper_bound: i64,
) -> Option<i64> {
    let mut rng = rand::thread_rng();

    let mut agents: Vec<Agent> = (0..pop_size)
        .map(|x| {
            let search_space = upper_bound - lower_bound;
            let pos = lower_bound + x as i64 * (search_space / (pop_size - 1) as i64);
            Agent {
                pos: pos as f64,
                fitness: func(pos),
            }
        })
        .collect();

    let mut iter = 0;
    let mut next_gen = Vec::with_capacity(agents.len());

    while iter < max_iter {
        if iter % 1000 == 0 {
            println!("{}: {:?}", iter, agents);
        }
        for agent in &agents {
            // Pick 3 agents at random to combine
            let others: Vec<&Agent> = agents.choose_multiple(&mut rng, 3).collect();
            let candidate_pos = others[0].pos + diff_weight * (others[1].pos - others[2].pos);
            let candidate_fitness = func(candidate_pos as i64);

            // Circuit breaker:
            if candidate_fitness == 0 {
                return Some(candidate_pos as i64);
            }

            match candidate_fitness <= agent.fitness {
                true => next_gen.push(Agent {
                    pos: candidate_pos,
                    fitness: candidate_fitness,
                }),
                false => next_gen.push(agent.clone()),
            }
        }
        agents.clear();
        agents = next_gen.drain(..).collect();
        iter += 1;
    }

    None
}
