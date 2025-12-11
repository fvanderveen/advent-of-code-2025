use std::collections::HashMap;
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{p, Grid};

pub const DAY7: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) -> Result<String, String> {
    let manifold = parse_input(input)?;
    let result = manifold.get_beam_split_count();

    Ok(format!("{}", result))
}
fn puzzle2(input: &String) -> Result<String, String> {
    let manifold = parse_input(input)?;
    let result = manifold.count_split_timelines();

    Ok(format!("{}", result))
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
enum Tile {
    Start,
    Splitter,
    #[default]
    Empty
}

type Manifold = Grid<Tile>;

impl Manifold {
    fn get_beam_split_count(&self) -> usize {
        // Beam starts – going downwards – at Tile::Start. When hitting a Splitter, splits into
        // two beams (left/right of the splitter) going down. Overlapping beams become a single beam.
        // Count how often a beam splits.

        // Since the beams move down the same time, we just keep track of the X coordinates of the
        // different beams.
        let start = *self.entries().iter().find_map(|(p, t)| if Tile::Start.eq(t) { Some(p) } else { None }).unwrap();
        let mut split_count = 0;

        let mut beams = vec![start.x];
        let mut y = start.y + 1;

        while y < self.bounds.height as isize {
            let mut new_beams = vec![];

            for beam in &beams {
                match self.get(&p((*beam, y))) {
                    Some(Tile::Splitter) => {
                        split_count += 1;

                        let left = beam - 1;
                        if !new_beams.contains(&left) { new_beams.push(left) }

                        let right = beam + 1;
                        if !new_beams.contains(&right) { new_beams.push(right) }
                    },
                    _ => if !new_beams.contains(beam) { new_beams.push(*beam) }
                }
            }

            y += 1;
            beams = new_beams;
        }

        split_count
    }

    fn count_split_timelines(&self) -> usize {
        // Beam starts – going downwards – at Tile::Start.
        // This time, the particle splits the timeline when hitting a splitter.
        // How many timelines do we create in total?

        let start = *self.entries().iter().find_map(|(p, t)| if Tile::Start.eq(t) { Some(p) } else { None }).unwrap();
        let mut timelines = 1;

        let mut beams: HashMap<isize, usize> = HashMap::new(); // Map of x-index to the amount of 'particles' travelling there in different timelines.
        beams.insert(start.x, 1);
        let mut y = start.y + 1;

        while y < self.bounds.height as isize {
            let mut new_beams = HashMap::new();

            for (beam, amount) in &beams {
                match self.get(&p((*beam, y))) {
                    Some(Tile::Splitter) => {
                        timelines += amount;

                        let left = beam - 1;
                        let right = beam + 1;

                        new_beams.insert(left, new_beams.get(&left).unwrap_or(&0) + amount);
                        new_beams.insert(right, new_beams.get(&right).unwrap_or(&0) + amount);
                    },
                    _ => { new_beams.insert(*beam, new_beams.get(beam).unwrap_or(&0) + amount); }
                }
            }

            y += 1;
            beams = new_beams;
        }

        timelines
    }
}

fn parse_input(input: &str) -> Result<Manifold, String> { input.parse() }

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Tile::Start),
            "^" => Ok(Tile::Splitter),
            "." => Ok(Tile::Empty),
            _ => Err(format!("Unknown tile: {}", s))
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Start => write!(f, "S"),
            Tile::Splitter => write!(f, "^"),
            Tile::Empty => write!(f, "."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day07::{parse_input, Tile};
    use crate::util::geometry::p;

    const EXAMPLE_INPUT: &str = "\
        .......S.......\n\
        ...............\n\
        .......^.......\n\
        ...............\n\
        ......^.^......\n\
        ...............\n\
        .....^.^.^.....\n\
        ...............\n\
        ....^.^...^....\n\
        ...............\n\
        ...^.^...^.^...\n\
        ...............\n\
        ..^...^.....^..\n\
        ...............\n\
        .^.^.^.^.^...^.\n\
        ...............\n\
    ";

    #[test]
    fn test_parse_input() {
        let res = parse_input(EXAMPLE_INPUT);

        assert!(res.is_ok());

        let manifold = res.unwrap();

        assert_eq!(manifold.get(&p((7, 0))), Some(Tile::Start));
    }

    #[test]
    fn test_get_beam_split_count() {
        let manifold = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(manifold.get_beam_split_count(), 21);
    }

    #[test]
    fn test_count_split_timelines() {
        let manifold = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(manifold.count_split_timelines(), 40);
    }
}