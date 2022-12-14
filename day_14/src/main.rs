use std::cmp;
use std::env;
use std::fs;

type Pos = (usize, usize);

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();

    let (mut map_p1, offset) = parse_input(&inp);
    let mut map_p2 = map_p1.clone();

    while let Some(settled_at) = drop_sand_abyss(&map_p1, (500 - offset.0, 0 - offset.1)) {
        map_p1[settled_at.1][settled_at.0] = 'o';
    }
    //print_map(&map_p1);
    let p1 = map_p1.iter().flatten().filter(|c| **c == 'o').count();
    println!("Part 1: {:?}", p1);

    // Extend the map horizontally to account for new rules
    const EXTEND_BY: usize = 200;
    for line in &mut map_p2 {
        let mut new = vec!['.'; EXTEND_BY];
        new.extend_from_slice(&line);
        new.extend(vec!['.'; EXTEND_BY]);
        *line = new;
    }
    let offset_2 = (offset.0 - EXTEND_BY, offset.1);
    // Add the floor
    map_p2.push(vec!['.'; map_p2[0].len()]);
    map_p2.push(vec!['#'; map_p2[0].len()]);

    let start = (500 - offset_2.0, 0 - offset_2.1);
    while let Some(settled_at) = drop_sand_abyss(&map_p2, start) {
        map_p2[settled_at.1][settled_at.0] = 'o';
        if settled_at == start {
            break;
        }
    }
    print_map(&map_p2);
    let p2 = map_p2.iter().flatten().filter(|c| **c == 'o').count();
    println!("Part 2: {:?}", p2);
}

fn parse_input(inp: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    // First, find all points to determine the size and offset of this map.
    let points: Vec<Vec<Pos>> = inp
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|coord| {
                    let parts: Vec<_> = coord
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect();
                    (parts[0], parts[1])
                })
                .collect()
        })
        .collect();

    let bottom_left = points
        .iter()
        .flatten()
        .fold((500, 0), |acc, pt| (cmp::min(acc.0, pt.0), 0));
    let top_right = points.iter().flatten().fold((500, 0), |acc, pt| {
        (cmp::max(acc.0, pt.0), cmp::max(acc.1, pt.1))
    });
    let width = top_right.0 - bottom_left.0 + 1;
    let height = top_right.1 - bottom_left.1 + 1;

    let mut map = vec![vec!['.'; width]; height];

    for line in points {
        for w in line.windows(2) {
            let (x1, x2, y1, y2) = (
                w[0].0 - bottom_left.0,
                w[1].0 - bottom_left.0,
                w[0].1,
                w[1].1,
            );
            if x1 < x2 {
                map[y1][x1..=x2].fill('#');
            } else if x2 < x1 {
                map[y1][x2..=x1].fill('#');
            } else if y1 < y2 {
                (y1..=y2).for_each(|y| map[y][x1] = '#');
            } else if y2 < y1 {
                (y2..=y1).for_each(|y| map[y][x1] = '#');
            } else {
                panic! {"Oh-oh! What do we do with {:?}?", w};
            }
        }
    }

    (map, bottom_left)
}

fn drop_sand_abyss(map: &Vec<Vec<char>>, source: (usize, usize)) -> Option<Pos> {
    let mut pos = source;

    loop {
        let next_y = pos.1 + 1;
        // Reached the bottom!
        if next_y >= map.len() {
            return None;
        }
        let next_line = &map[pos.1 + 1];
        match next_line[pos.0 - 1..=pos.0 + 1] {
            [_, '.', _] => (),         // Straight down
            ['.', _, _] => pos.0 -= 1, // Left
            [_, _, '.'] => pos.0 += 1, // Right
            _ => return Some(pos),     // Settled
        }
        pos.1 += 1;
    }
}

fn drop_sand_invis_floor(map: &Vec<Vec<char>>, source: (usize, usize)) -> Option<Pos> {
    let mut pos = source;

    loop {
        let next_y = pos.1 + 1;
        // Reached the bottom!
        if next_y >= map.len() {
            return None;
        }
        let next_line = &map[pos.1 + 1];
        match next_line[pos.0 - 1..=pos.0 + 1] {
            [_, '.', _] => (),         // Straight down
            ['.', _, _] => pos.0 -= 1, // Left
            [_, _, '.'] => pos.0 += 1, // Right
            _ => return Some(pos),     // Settled
        }
        pos.1 += 1;
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    map.iter().for_each(|l| {
        l.iter().for_each(|c| print!("{}", c));
        print!("\n");
    });
}
