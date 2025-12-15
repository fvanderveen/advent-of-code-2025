use crate::days::Day;
use crate::util::geometry::Point3D;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub const DAY8: Day = Day { puzzle1, puzzle2 };

fn puzzle1(input: &String) -> Result<String, String> {
    let points = parse_input(input)?;

    let circuits = connect_shortest_paths(&points, 1000);

    let result = circuits[0].len() * circuits[1].len() * circuits[2].len();

    Ok(format!("{}", result))
}
fn puzzle2(input: &String) -> Result<String, String> {
    let points = parse_input(input)?;

    let last_pair = find_last_connecting_pair(&points).ok_or("No last connecting pair!".to_string())?;

    let result = last_pair.p1.x * last_pair.p2.x;

    Ok(format!("{}", result))
}

fn parse_input(input: &str) -> Result<Vec<Point3D>, String> {
    input.lines().map(|l| l.parse()).collect()
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Circuit {
    points: Vec<Point3D>,
}

impl Circuit {
    fn merge(left: &Self, right: &Self) -> Self {
        Self {
            points: left
                .points
                .iter()
                .chain(right.points.iter())
                .copied()
                .collect(),
        }
    }

    fn len(&self) -> usize {
        self.points.len()
    }
}

fn connect_shortest_paths(points: &Vec<Point3D>, amount: usize) -> Vec<Circuit> {
    let mut jumper_pairs = JumperBoxPair::create_pairs(points);
    let mut res: Vec<Circuit> = Vec::new();

    // Now take amount pairs and connect them to circuits
    // Options:
    // - neither point is part of an existing circuit -> create new
    // - one point is part of circuit -> add to that
    // - both points are part of _different_ circuits -> merge them
    // - both points are part of the same circuit -> ignore (does this count as connecting though? I think so)
    let mut to_connect = amount;
    while to_connect > 0 {
        if let Some(pair) = jumper_pairs.pop() {
            to_connect -= 1;

            pair.connect(&mut res);
        }
    }

    res.sort_by(|a, b| b.len().cmp(&a.len()));

    res
}

fn find_last_connecting_pair(points: &Vec<Point3D>) -> Option<JumperBoxPair> {
    // Puzzle 2 wants us to keep connecting pairs until we get to the pair that ensures all points
    // are in one big circuit.

    let mut jumper_pairs = JumperBoxPair::create_pairs(points);
    let mut circuits: Vec<Circuit> = Vec::new();

    while let Some(pair) = jumper_pairs.pop() {
        pair.connect(&mut circuits);

        if circuits.len() == 1 && circuits[0].points.len() == points.len() {
            // We hit the end condition, return the current points:
            return Some(pair)
        }
    }

    None
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct JumperBoxPair {
    p1: Point3D,
    p2: Point3D,
}

impl JumperBoxPair {
    fn create_pairs(points: &Vec<Point3D>) -> BinaryHeap<Self> {
        let mut jumper_pairs: BinaryHeap<JumperBoxPair> = BinaryHeap::new();
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                jumper_pairs.push(JumperBoxPair {
                    p1: points[i],
                    p2: points[j],
                });
            }
        }

        jumper_pairs
    }

    fn len(&self) -> f64 {
        self.p1.euclidean_distance(&self.p2)
    }

    fn connect(&self, circuits: &mut Vec<Circuit>) {
        let Self { p1, p2 } = self;

        let circuit_1 = circuits.iter().position(|c| c.points.contains(p1));
        let circuit_2 = circuits.iter().position(|c| c.points.contains(p2));

        match (circuit_1, circuit_2) {
            (None, None) => circuits.push(Circuit {
                points: vec![*p1, *p2],
            }),
            (Some(ca), None) => circuits.get_mut(ca).unwrap().points.push(*p2),
            (None, Some(cb)) => circuits.get_mut(cb).unwrap().points.push(*p1),
            (Some(ca), Some(cb)) if ca == cb => {
                /* already part of the same circuit, do nothing */
            }
            (Some(ca), Some(cb)) => {
                // Remove both existing circuits, and push a new, merged, one.
                let first = circuits.remove(ca.max(cb));
                let second = circuits.remove(ca.min(cb));
                circuits.push(Circuit::merge(&first, &second));
            }
        }
    }
}

impl Ord for JumperBoxPair {
    fn cmp(&self, other: &Self) -> Ordering {
        let len = self.len();
        let other_len = other.len();

        if len.gt(&other_len) {
            Ordering::Less
        } else if len.lt(&other_len) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for JumperBoxPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day08::{connect_shortest_paths, find_last_connecting_pair, parse_input, JumperBoxPair};
    use crate::util::geometry::Point3D;

    const EXAMPLE_INPUT: &str = "\
        162,817,812\n\
        57,618,57\n\
        906,360,560\n\
        592,479,940\n\
        352,342,300\n\
        466,668,158\n\
        542,29,236\n\
        431,825,988\n\
        739,650,466\n\
        52,470,668\n\
        216,146,977\n\
        819,987,18\n\
        117,168,530\n\
        805,96,715\n\
        346,949,466\n\
        970,615,88\n\
        941,993,340\n\
        862,61,35\n\
        984,92,344\n\
        425,690,689\n\
    ";

    #[test]
    fn test_parse_input() {
        let res = parse_input(EXAMPLE_INPUT);

        assert!(res.is_ok());

        let points = res.unwrap();

        assert_eq!(points.len(), 20);
        assert_eq!(
            points[0],
            Point3D {
                x: 162,
                y: 817,
                z: 812
            }
        );
    }

    #[test]
    fn test_connect_shortest_paths() {
        let res = connect_shortest_paths(&parse_input(EXAMPLE_INPUT).unwrap(), 10);

        assert_eq!(res[0].points.len(), 5);
        assert_eq!(res[1].points.len(), 4);
        assert_eq!(res[2].points.len(), 2);
    }

    #[test]
    fn test_find_last_connecting_pair() {
        let res = find_last_connecting_pair(&parse_input(EXAMPLE_INPUT).unwrap());

        assert_eq!(res, Some(JumperBoxPair {
            p1: (216,146,977).into(),
            p2: (117,168,530).into()
        }))
    }
}
