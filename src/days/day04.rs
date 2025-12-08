use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY4: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) -> Result<String, String> {
    let map = input.parse::<Map>()?;
    let result = map.get_moveable_paper_count();

    Ok(format!("{}", result))
}
fn puzzle2(input: &String) -> Result<String, String> {
    let map = input.parse::<Map>()?;
    let result = map.get_removable_paper_count();

    Ok(format!("{}", result))
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
enum Tile {
    #[default]
    Empty,
    Paper
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "@" => Ok(Tile::Paper),
            "." | " " => Ok(Tile::Empty),
            _ => Err(format!("Unknown tile: {}", s))
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Paper => write!(f, "@"),
        }
    }
}

type Map = Grid<Tile>;

impl Map {
    fn get_movable_papers(&self) -> Vec<Point> {
        let mut result = vec![];

        for (p, t) in self.entries() {
            if t == Tile::Empty { continue; }

            let adjacent = self.get_adjacent(&p, Directions::All).into_iter().filter(|t| *t == Tile::Paper).count();
            if adjacent < 4 { result.push(p) }
        }

        result
    }

    fn get_moveable_paper_count(&self) -> usize {
        self.get_movable_papers().iter().count()
    }

    fn get_removable_paper_count(&self) -> usize {
        let mut state = self.clone();
        let mut removed = 0;

        loop {
            let to_remove = state.get_movable_papers();
            if to_remove.is_empty() { break; }

            removed += to_remove.len();
            for p in to_remove {
                state.set(p, Tile::Empty);
            }
        }

        removed
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day04::{Map, Tile};
    use crate::util::geometry::p;

    const EXAMPLE_INPUT: &str = "\
        ..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.\n\
    ";

    #[test]
    fn test_parse_map() {
        let res = EXAMPLE_INPUT.parse::<Map>();

        assert!(res.is_ok());

        let map = res.unwrap();
        assert_eq!(map.get(&p((0, 0))), Some(Tile::Empty));
        assert_eq!(map.get(&p((1, 1))), Some(Tile::Paper));
    }

    #[test]
    fn test_get_moveable_paper_count() {
        let map = EXAMPLE_INPUT.parse::<Map>().unwrap();

        assert_eq!(map.get_moveable_paper_count(), 13);
    }

    #[test]
    fn test_get_removable_paper_count() {
        let map = EXAMPLE_INPUT.parse::<Map>().unwrap();

        assert_eq!(map.get_removable_paper_count(), 43);
    }
}