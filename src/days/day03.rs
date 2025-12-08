use std::str::FromStr;
use crate::days::Day;
use crate::util::number::parse_usize;

pub const DAY3: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) -> Result<String, String> {
    let banks = parse_input(input)?;

    let result = banks.iter().map(|b| b.get_largest_joulage()).sum::<usize>();
    Ok(format!("{}", result))
}
fn puzzle2(input: &String) -> Result<String, String> {
    let banks = parse_input(input)?;

    let result = banks.iter().map(|b| b.get_overcharge_joulage()).sum::<usize>();
    Ok(format!("{}", result))
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct BatteryBank {
    values: Vec<usize>
}

impl BatteryBank {
    fn get_largest_joulage(&self) -> usize {
        // Find the batteries A,B (in order) so that AB is the highest value.
        let mut first = 0;
        let mut second  = 0;

        for i in 0..self.values.len() {
            let current = self.values[i];
            let next = self.values.get(i+1);

            if current > first && let Some(v) = next {
                first = current;
                second = *v;
            } else if current > second {
                second = current;
            }
        }

        first * 10 + second
    }

    fn get_overcharge_joulage(&self) -> usize {
        // For the overcharge variant, we need 12 batteries (in order) forming the largest number.
        // For each number (N) we can:
        // - Init Y to 0
        // - take the range of Y..Z (where Z is len() - (12 - N))
        // - find the max index (Y)

        let mut values: [usize;12] = [0;12];

        let mut start_idx = 0;
        for digit in 0..12 {
            let max_idx = self.values.len() - (12 - digit);
            for idx in start_idx..=max_idx {
                if self.values[idx] > values[digit] {
                    values[digit] = self.values[idx];
                    start_idx = idx + 1;
                }
            }
        }

        // Build the resulting value
        let mut result = 0;
        for i in 0..values.len() {
            result *= 10;
            result += values[i];
        }

        result
    }
}

impl FromStr for BatteryBank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { values: s.chars().map(|c| parse_usize(&c.to_string())).collect::<Result<Vec<_>, _>>()? })
    }
}

fn parse_input(input: &str) -> Result<Vec<BatteryBank>, String> {
    input.lines().map(|l| l.parse::<BatteryBank>()).collect()
}

#[cfg(test)]
mod tests {
    use crate::days::day03::{parse_input, BatteryBank};

    const EXAMPLE_INPUT: &str = "\
        987654321111111\n\
        811111111111119\n\
        234234234234278\n\
        818181911112111\n\
    ";

    #[test]
    fn test_parse_input() {
        let res = parse_input(EXAMPLE_INPUT);

        assert!(res.is_ok());

        let banks = res.unwrap();

        assert_eq!(banks.len(), 4);
        assert_eq!(banks[0], BatteryBank { values: vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1] });
    }

    #[test]
    fn test_get_largest_joulage() {
        let banks = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(banks[0].get_largest_joulage(), 98);
        assert_eq!(banks[1].get_largest_joulage(), 89);
        assert_eq!(banks[2].get_largest_joulage(), 78);
        assert_eq!(banks[3].get_largest_joulage(), 92);
    }

    #[test]
    fn test_get_overcharge_joulage() {
        let banks = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(banks[0].get_overcharge_joulage(), 987654321111);
        assert_eq!(banks[1].get_overcharge_joulage(), 811111111119);
        assert_eq!(banks[2].get_overcharge_joulage(), 434234234278);
        assert_eq!(banks[3].get_overcharge_joulage(), 888911112111);
    }
}