use std::collections::HashSet;
use std::env;
use std::fs;
use std::ops::RangeInclusive;

type Pos = (i64, i64);

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    range: i64,
}

impl Sensor {
    fn coverage_at_y(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let dy = (self.pos.1 - y).abs();
        let half_width_at_y = self.range - dy;
        match half_width_at_y {
            w if w >= 0 => Some((self.pos.0 - half_width_at_y)..=(self.pos.0 + half_width_at_y)),
            _ => None,
        }
    }

    fn covers(&self, pt: Pos) -> bool {
        let dist = (self.pos.0 - pt.0).abs() + (self.pos.1 - pt.1).abs();
        dist <= self.range
    }

    fn boundary(&self) -> Vec<Pos> {
        let mut res = Vec::new();
        for x in (self.pos.0 - self.range - 1)..=(self.pos.0 + self.range + 1) {
            let xdist = (self.pos.0 - x).abs();
            let offset = self.range + 1 - xdist;
            res.push((x, self.pos.1 + offset));
            if offset > 0 {
                res.push((x, self.pos.1 - offset));
            }
        }
        res
    }
}

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

fn main() {
    let inp = get_input_contents();
    let (sensors, beacons) = parse_input(&inp);

    const P1_ROW: i64 = 2000000;
    let mut covered = line_coverage(&sensors, P1_ROW);
    // Don't count beacons!
    beacons
        .iter()
        .filter(|(_x, y)| *y == P1_ROW)
        .for_each(|(x, _y)| {
            covered.remove(x);
        });
    println!("Part 1: {}", covered.len());

    // Check all boundary points within the limits:
    let boundary_points: Vec<Pos> = sensors.iter().flat_map(|s| s.boundary()).collect();
    const LIMIT: i64 = 4000000;
    let distress_beacon = boundary_points
        .iter()
        .filter(|(x, y)| *x >= 0 && *x <= LIMIT && *y >= 0 && *y <= LIMIT)
        .filter(|pt| sensors.iter().all(|s| !s.covers(**pt)))
        .nth(0)
        .unwrap();
    println!("Part 2: {}", distress_beacon.0 * LIMIT + distress_beacon.1);
}

fn line_coverage(sensors: &Vec<Sensor>, line: i64) -> HashSet<i64> {
    sensors
        .iter()
        .filter_map(|s| s.coverage_at_y(line))
        .flatten()
        .collect()
}

fn parse_input(inp: &str) -> (Vec<Sensor>, Vec<Pos>) {
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    for l in inp.lines() {
        let parts: Vec<_> = l.split_whitespace().collect();
        let sx = parts[2]
            .split("=")
            .nth(1)
            .unwrap()
            .trim_end_matches(',')
            .parse::<i64>()
            .unwrap();
        let sy = parts[3]
            .split("=")
            .nth(1)
            .unwrap()
            .trim_end_matches(':')
            .parse::<i64>()
            .unwrap();
        let bx = parts[8]
            .split("=")
            .nth(1)
            .unwrap()
            .trim_end_matches(',')
            .parse::<i64>()
            .unwrap();
        let by = parts[9]
            .split("=")
            .nth(1)
            .unwrap()
            .trim_end_matches(':')
            .parse::<i64>()
            .unwrap();

        sensors.push(Sensor {
            pos: (sx, sy),
            range: ((sx - bx).abs() + (sy - by).abs()),
        });
        beacons.push((bx, by));
    }
    (sensors, beacons)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coverage_at_y() {
        let s = Sensor {
            pos: (8, 7),
            range: 9,
        };

        assert_eq!(s.coverage_at_y(7), Some(-1..=17));
        assert_eq!(s.coverage_at_y(10), Some(2..=14));
        assert_eq!(s.coverage_at_y(16), Some(8..=8));
    }

    #[test]
    fn test_covers() {
        let s = Sensor {
            pos: (8, 7),
            range: 9,
        };

        assert_eq!(s.covers((8, 7)), true);
        assert_eq!(s.covers((8, 16)), true);
        assert_eq!(s.covers((8, 17)), false);
        assert_eq!(s.covers((2, 10)), true);
        assert_eq!(s.covers((1, 10)), false);
    }

    #[test]

    fn test_boundary() {
        let s = Sensor {
            pos: (0, 0),
            range: 1,
        };
        assert_eq!(
            s.boundary(),
            vec![
                (-2, 0),
                (-1, 1),
                (-1, -1),
                (0, 2),
                (0, -2),
                (1, 1),
                (1, -1),
                (2, 0)
            ]
        );
    }
}
