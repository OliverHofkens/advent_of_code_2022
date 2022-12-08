use std::env;
use std::fs;
use take_until::TakeUntilExt;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();

    let height_map: Vec<Vec<i8>> = inp
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
        .collect();

    let visibility = build_visibility_map(&height_map);
    println!(
        "Puzzle 1: {}",
        visibility
            .iter()
            .map(|l| l.iter().map(|b| *b as u64).sum::<u64>())
            .sum::<u64>()
    );

    let scores = build_scenic_score_map(&height_map);
    println!(
        "Puzzle 2: {}",
        scores
            .iter()
            .map(|l| l.iter().max().unwrap())
            .max()
            .unwrap()
    );
}

fn build_visibility_map(height_map: &Vec<Vec<i8>>) -> Vec<Vec<bool>> {
    let mut visible: Vec<Vec<bool>> = vec![vec![false; height_map[0].len()]; height_map.len()];

    for (i, map_line) in height_map.iter().enumerate() {
        let mut max_seen: i8 = -1;
        for (j, height) in map_line.iter().enumerate() {
            if *height > max_seen {
                visible[i][j] = true;
                max_seen = *height;
            }
            if *height == 9 {
                break;
            }
        }

        max_seen = -1;
        for (j, height) in map_line.iter().rev().enumerate() {
            let idx = map_line.len() - 1 - j;
            if *height > max_seen {
                visible[i][idx] = true;
                max_seen = *height;
            }
            if *height == 9 {
                break;
            }
        }
    }

    for j in 0..height_map[0].len() {
        let map_col = height_map.iter().map(|line| &line[j]);
        let mut max_seen: i8 = -1;

        for (i, height) in map_col.clone().enumerate() {
            if *height > max_seen {
                visible[i][j] = true;
                max_seen = *height;
            }
            if *height == 9 {
                break;
            }
        }

        max_seen = -1;
        for (i, height) in map_col.rev().enumerate() {
            let idx = height_map.len() - 1 - i;
            if *height > max_seen {
                visible[idx][j] = true;
                max_seen = *height;
            }
            if *height == 9 {
                break;
            }
        }
    }

    visible
}

fn build_scenic_score_map(height_map: &Vec<Vec<i8>>) -> Vec<Vec<u64>> {
    let mut scores: Vec<Vec<u64>> = vec![vec![0; height_map[0].len()]; height_map.len()];

    for (i, map_line) in height_map.iter().enumerate() {
        if i == 0 || i == height_map.len() - 1 {
            continue;
        }
        for (j, height) in map_line.iter().enumerate() {
            if j == 0 || j == map_line.len() - 1 {
                continue;
            }

            let vis_right = map_line[j + 1..]
                .iter()
                .take_until(|t| *t >= height)
                .count();
            let vis_left = map_line
                .iter()
                .rev()
                .skip(map_line.len() - j)
                .take_until(|t| *t >= height)
                .count();
            let vis_down = height_map[i + 1..]
                .iter()
                .map(|l| l[j])
                .take_until(|t| t >= height)
                .count();
            let vis_up = height_map
                .iter()
                .rev()
                .skip(height_map.len() - i)
                .map(|l| l[j])
                .take_until(|t| t >= height)
                .count();

            println!(
                "{},{}: {}*{}*{}*{}",
                i, j, vis_up, vis_left, vis_right, vis_down
            );
            scores[i][j] = (vis_right * vis_left * vis_down * vis_up) as u64;
        }
    }

    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_visibility_map() {
        let inp = [
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        let exp: Vec<Vec<bool>> = [
            [true, true, true, true, true],
            [true, true, true, false, true],
            [true, true, false, true, true],
            [true, false, true, false, true],
            [true, true, true, true, true],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();

        let res = build_visibility_map(&inp);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_build_score_map() {
        let inp = [
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        let exp: Vec<Vec<u64>> = [
            [0, 0, 0, 0, 0],
            [0, 1, 4, 1, 0],
            [0, 6, 1, 2, 0],
            [0, 1, 8, 3, 0],
            [0, 0, 0, 0, 0],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();

        let res = build_scenic_score_map(&inp);
        assert_eq!(res, exp);
    }
}
