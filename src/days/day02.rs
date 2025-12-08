use std::ops::{RangeInclusive};
use crate::days::Day;
use crate::util::parser::Parser;

pub const DAY2: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) -> Result<String, String> {
    let data = parse_input(input)?;
    let sum = sum_invalid_ids(&data, &is_invalid_id);

    Ok(format!("{}", sum))
}
fn puzzle2(input: &String) -> Result<String, String> {
    let data = parse_input(input)?;
    let sum = sum_invalid_ids(&data, &is_invalid_id_p2);

    Ok(format!("{}", sum))
}

fn sum_invalid_ids(ranges: &Vec<RangeInclusive<usize>>, check: &dyn Fn(usize) -> bool) -> usize {
    // It's just day 2, just iterating all ids should be good... right?

    let mut result = 0;

    for range in ranges {
        for id in range.clone() {
            if check(id) {
                result += id;
            }
        }
    }

    result
}

fn is_invalid_id(id: usize) -> bool {
    // id is invalid if, when split in the middle, yields two the same values.
    // e.g.: 55, 1010, 123123, 5588055880, etc.
    let str = id.to_string();
    if (str.len() % 2) == 1 {
        // uneven amount of digits is not invalid
        false
    } else {
        let (first, second) = str.split_at(str.len() / 2);
        first == second
    }
}

fn is_invalid_id_p2(id: usize) -> bool {
    // For part 2, and ID is invalid if it is a repetition of any amount.
    // i.e. 123123 (123 x 2), 121212 (12 x 3), etc.

    let str = id.to_string();
    let max_len = str.len() / 2;

    'no_match: for i in 1..=max_len {
        if (str.len() % i) != 0 { continue; } // Cannot chunk with this length

        let pattern = &str[0..i];
        for j in (i..str.len()).step_by(i) {
            let check = &str[j..j+i];
            if pattern != check { continue 'no_match }
        }

        return true;
    }

    false
}

fn parse_input(input: &str) -> Result<Vec<RangeInclusive<usize>>, String> {
    let mut result = vec![];
    let mut parser = Parser::new(input);

    while !parser.is_exhausted() {
        if !result.is_empty() { parser.literal(",")?; }

        let start = parser.usize()?;
        parser.literal("-")?;
        let end = parser.usize()?;
        result.push(start..=end);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::days::day02::{is_invalid_id, is_invalid_id_p2, parse_input, sum_invalid_ids};

    const EXAMPLE_INPUT: &str = "\
    11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\n\
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,\n\
    824824821-824824827,2121212118-2121212124\
    ";

    #[test]
    fn test_parse_input() {
        let res = parse_input(EXAMPLE_INPUT);

        assert!(res.is_ok());

        let ranges = res.unwrap();

        assert_eq!(ranges.len(), 11);
        assert_eq!(ranges[0], 11..=22);
        assert_eq!(ranges[3], 1188511880..=1188511890);
    }

    #[test]
    fn test_is_invalid_id() {
        assert_eq!(is_invalid_id(11), true);
        assert_eq!(is_invalid_id(123123), true);
        assert_eq!(is_invalid_id(1188511885), true);
        assert_eq!(is_invalid_id(123459876123459876), true);

        assert_eq!(is_invalid_id(101), false);
        assert_eq!(is_invalid_id(5005), false);
        assert_eq!(is_invalid_id(123456789987654321), false);
    }

    #[test]
    fn test_sum_invalid_ids() {
        let data = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(sum_invalid_ids(&data, &is_invalid_id), 1227775554);
        assert_eq!(sum_invalid_ids(&data, &is_invalid_id_p2), 4174379265);
    }

    #[test]
    fn test_is_invalid_id_p2() {
        assert_eq!(is_invalid_id_p2(11), true);
        assert_eq!(is_invalid_id_p2(111), true);
        assert_eq!(is_invalid_id_p2(111111111), true);
        assert_eq!(is_invalid_id_p2(123123), true);
        assert_eq!(is_invalid_id_p2(123123123), true);
        assert_eq!(is_invalid_id_p2(1234512345), true);

        assert_eq!(is_invalid_id_p2(11112), false);
        assert_eq!(is_invalid_id_p2(1231234), false);
    }
}