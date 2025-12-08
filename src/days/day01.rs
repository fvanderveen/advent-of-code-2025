use std::str::FromStr;
use crate::days::Day;
use crate::util::parser::Parser;

pub const DAY1: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) -> Result<String, String> {
    let rotations = parse_input(input)?;
    let result = count_stops_on_0(&rotations);

    Ok(format!("{}", result))
}
fn puzzle2(input: &String) -> Result<String, String> {
    let rotations = parse_input(input)?;
    let result = count_click_on_0(&rotations);

    Ok(format!("{}", result))
}

fn count_stops_on_0(rotations: &Vec<Rotation>) -> usize {
    let mut result = 0;

    let mut current = 50;

    for rot in rotations {
        match rot {
            Rotation { direction: Direction::Left, amount: left } => current = (current + 100 - (left % 100)) % 100,
            Rotation { direction: Direction::Right, amount: right } => current = (current + right) % 100,
        }

        if current == 0 { result += 1; }
    }

    result
}

fn count_click_on_0(rotations: &Vec<Rotation>) -> usize {
    let mut result = 0;

    // Any click, whether passing or stopping, on 0 counts for part 2.
    let mut current = 50;

    for rot in rotations {
        result += rot.amount / 100; // Any full rotation passes 0
        let move_current = rot.amount % 100;

        if current != 0 && move_current > 0 {
            // When not already on 0, the move might move past or to 0:
            if rot.direction == Direction::Left && move_current >= current { result += 1 }
            if rot.direction == Direction::Right && current + move_current >= 100 { result += 1 }
        }

        match rot.direction {
            Direction::Left => current = ((current + 100) - move_current) % 100,
            Direction::Right => current = (current + move_current) % 100,
        }
    }

    result
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Left, Right
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("Unknown direction: {}", s))
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Rotation {
    direction: Direction,
    amount: usize
}

fn parse_input(input: &str) -> Result<Vec<Rotation>, String> {
    input.lines().map(|l| parse_line(l)).collect()
}

fn parse_line(input: &str) -> Result<Rotation, String> {
    let mut parser = Parser::new(input);
    let dir = parser.one_of(vec!["L", "R"])?;
    let amount =  parser.usize()?;
    parser.ensure_exhausted()?;

    Ok(Rotation { amount, direction: dir.parse()? })
}

#[cfg(test)]
mod tests {
    use crate::days::day01::{count_click_on_0, count_stops_on_0, parse_input, Direction, Rotation};

    const EXAMPLE_INPUT: &str = "\
        L68\n\
        L30\n\
        R48\n\
        L5\n\
        R60\n\
        L55\n\
        L1\n\
        L99\n\
        R14\n\
        L82\n\
    ";

    #[test]
    fn test_parse_input() {
        let res = parse_input(EXAMPLE_INPUT);

        assert!(res.is_ok());

        let rotations = res.unwrap();

        assert_eq!(rotations[0], Rotation { direction: Direction::Left, amount: 68 });
        assert_eq!(rotations[1], Rotation { direction: Direction::Left, amount: 30 });
        assert_eq!(rotations[2], Rotation { direction: Direction::Right, amount: 48 });
    }

    #[test]
    fn test_count_stops_on_0() {
        let rotations = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(count_stops_on_0(&rotations), 3);
    }

    #[test]
    fn test_count_click_on_0() {
        let rotations = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(count_click_on_0(&rotations), 6);
    }

    #[test]
    fn test_count_click_on_0_full_rotations() {
        let rotations = vec![
            Rotation { direction: Direction::Left, amount: 1000 }, // 10 full spins; stays at 50
            Rotation { direction: Direction::Right, amount: 50 }, // 0 full spins; ends at 0
            Rotation { direction: Direction::Right, amount: 100 }, // 1 full spin, ends at 0
            Rotation { direction: Direction::Right, amount: 10 }, // 0 full spins, no passing 0
            Rotation { direction: Direction::Left, amount: 120 }, // 1 full spin, also passes 0
        ];

        assert_eq!(count_click_on_0(&rotations), 14);
    }
}