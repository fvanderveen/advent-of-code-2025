use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::RangeInclusive;
use crate::days::Day;
use crate::util::number::parse_usize;
use crate::util::parser::Parser;

pub const DAY5: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) -> Result<String, String> {
    let inventory = parse_input(input)?;
    let fresh_ingredients = inventory.get_fresh_ingredients().len();

    Ok(format!("{}", fresh_ingredients))
}
fn puzzle2(input: &String) -> Result<String, String> {
    let inventory = parse_input(input)?;
    let total_fresh_ids = inventory.get_total_fresh_ingredient_ids();

    Ok(format!("{}", total_fresh_ids))
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct InventoryManagement {
    fresh_ingredients: Vec<RangeInclusive<usize>>,
    available_ingredients: Vec<usize>,
}

impl InventoryManagement {
    fn get_fresh_ingredients(&self) -> Vec<&usize> {
        self.available_ingredients.iter().filter(|v| self.fresh_ingredients.iter().any(|r| r.contains(v))).collect()
    }

    fn get_total_fresh_ingredient_ids(&self) -> usize {
        // Count the unique ids in all ranges that are considered valid.
        let mut bag :BinaryHeap<Delimiter> = BinaryHeap::new();

        for range in &self.fresh_ingredients {
            bag.push(Delimiter::Start(range.start()));
            bag.push(Delimiter::End(range.end()));
        }

        let mut current_start = 0;
        let mut current_level = 0;
        let mut total_count = 0;

        while let Some(delim) = bag.pop() {
            match delim {
                Delimiter::Start(v) if current_level == 0 => {
                    current_start = *v;
                    current_level = 1;
                }
                Delimiter::Start(_) => {
                    current_level += 1;
                }
                Delimiter::End(v) => {
                    current_level -= 1;
                    if current_level == 0 {
                        total_count += (v - current_start) + 1; // ranges are inclusive.
                    }
                }
            }
        }

        total_count
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Delimiter<'a> {
    Start(&'a usize),
    End(&'a usize)
}

impl Ord for Delimiter<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let (value, is_start) = match self { Self::Start(v) => (v, true), Self::End(v) => (v, false) };
        let (other_value, other_is_start) = match other { Self::Start(v) => (v, true), Self::End(v) => (v, false) };

        other_value.cmp(value).then(other_is_start.cmp(&is_start))
    }
}

impl PartialOrd for Delimiter<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Result<InventoryManagement, String> {
    let sanitized_input = input.replace("\r\n", "\n");
    let (fresh, available) = sanitized_input.split_once("\n\n").ok_or("Invalid input, could not split on a blank line!".to_string())?;

    let mut fresh_ingredients = Vec::new();

    let mut fresh_parser = Parser::new(fresh);
    while !fresh_parser.is_exhausted() {
        let range_start = fresh_parser.usize()?;
        fresh_parser.literal("-")?;
        let range_end = fresh_parser.usize()?;

        fresh_ingredients.push(range_start..=range_end);
    }

    let available_ingredients = available.lines().map(|l| parse_usize(l)).collect::<Result<Vec<_>, _>>()?;

    Ok(InventoryManagement { fresh_ingredients, available_ingredients })
}

#[cfg(test)]
mod tests {
    use crate::days::day05::{parse_input, Delimiter};

    const EXAMPLE_INPUT: &str = "\
        3-5\n\
        10-14\n\
        16-20\n\
        12-18\n\
        \n\
        1\n\
        5\n\
        8\n\
        11\n\
        17\n\
        32\n\
    ";

    #[test]
    fn test_parse_input() {
        let res = parse_input(EXAMPLE_INPUT);

        assert!(res.is_ok());

        let inventory = res.unwrap();

        assert_eq!(inventory.fresh_ingredients.len(), 4);
        assert_eq!(inventory.fresh_ingredients[0], 3..=5);
        assert_eq!(inventory.fresh_ingredients[1], 10..=14);

        assert_eq!(inventory.available_ingredients, vec![1,5,8,11,17,32]);
    }

    #[test]
    fn test_get_fresh_ingredients() {
        let inventory = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(inventory.get_fresh_ingredients(), vec![&5, &11, &17])
    }

    #[test]
    fn test_get_total_fresh_ingredients() {
        let inventory = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(inventory.get_total_fresh_ingredient_ids(), 14)
    }

    #[test]
    fn test_delimiter_ordering() {
        // Note: greater values get consumed first with BinaryHeap
        assert!(Delimiter::Start(&0).gt(&Delimiter::Start(&1)));
        assert!(Delimiter::Start(&0).gt(&Delimiter::End(&1)));
        assert!(Delimiter::Start(&10).lt(&Delimiter::End(&10)));
        assert!(Delimiter::End(&10).gt(&Delimiter::End(&12)));
    }
}