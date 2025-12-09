use std::str::FromStr;
use crate::days::Day;
use crate::util::number::parse_usize;
use crate::util::parser::Parser;

pub const DAY6: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) -> Result<String, String> {
    let problems = parse_input(input)?;
    let result = problems.iter().map(|p| p.solve()).sum::<usize>();

    Ok(format!("{}", result))
}
fn puzzle2(input: &String) -> Result<String, String> {
    let problems = parse_input_p2(input)?;
    let result = problems.iter().map(|p| p.solve()).sum::<usize>();

    Ok(format!("{}", result))
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct MathProblem {
    operator: Operator,
    values: Vec<usize>
}

impl MathProblem {
    fn solve(&self) -> usize {
        let mut result = self.values[0];
        for value in &self.values[1..] {
            result = self.operator.apply(result, *value);
        }
        result
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Operator {
    Add,
    Multiply
}

impl Operator {
    fn apply(&self, left: usize, right: usize) -> usize {
        match self {
            Operator::Add => left + right,
            Operator::Multiply => left * right
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<MathProblem>, String> {
    // input is in a number of columns, with the last line containing the operators.
    // Probably the easiest to consume numbers one by one from each line :shrug:

    let mut result = vec![];
    let mut parsers = input.lines().map(|l| Parser::new(l)).collect::<Vec<_>>();

    let operator_idx = parsers.len()-1;
    let numbers_range = 0..operator_idx;

    while !parsers[operator_idx].is_exhausted() {
        let numbers = parsers[numbers_range.clone()].iter_mut().map(|p| p.usize()).collect::<Result<Vec<_>, _>>()?;
        let operator = parsers[operator_idx].str(1)?.parse::<Operator>()?;

        result.push(MathProblem { operator, values: numbers });
    }

    Ok(result)
}

fn parse_input_p2(input: &str) -> Result<Vec<MathProblem>, String> {
    // Numbers are written top-to-bottom, right to left. Whitespace matters.
    let lines = input.lines().collect::<Vec<_>>();
    let numbers = &lines[0..lines.len()-1];
    let operators = lines[lines.len()-1];

    let mut result = vec![];
    let mut column_idx = 0;

    loop {
        if column_idx >= operators.len() { break; }

        let operator = operators[column_idx..column_idx+1].parse::<Operator>()?;
        let mut values = vec![];

        while let Some(number) = get_number_at(numbers, column_idx) {
            values.push(number);
            column_idx += 1;
        }

        result.push(MathProblem { operator, values });
        column_idx += 1; // Skip over the empty column found
    }

    Ok(result)
}

fn get_number_at(lines: &[&str], idx: usize) -> Option<usize> {
    let digits = lines.iter().filter_map(|l| match l.chars().nth(idx) {
        Some(c) if c.is_digit(10) => Some(parse_usize(&c.to_string()).unwrap()),
        _ => None
    }).collect::<Vec<_>>();

    digits.into_iter().reduce(|l, r| (l * 10) + r)
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(format!("Unknown operator: {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day06::{parse_input, parse_input_p2, MathProblem, Operator};

    const EXAMPLE_INPUT: &str = "\
        123 328  51 64 \n\
        \u{20}45 64  387 23 \n\
        \u{20} 6 98  215 314\n\
        *   +   *   +  \n\
    ";

    #[test]
    fn test_parse_input() {
        let res = parse_input(EXAMPLE_INPUT);

        assert!(res.is_ok(), "{:?}", res.unwrap_err());

        let problems = res.unwrap();
        assert_eq!(problems.len(), 4);
        assert_eq!(problems[0], MathProblem { operator: Operator::Multiply, values: vec![123, 45, 6] });
        assert_eq!(problems[3], MathProblem { operator: Operator::Add, values: vec![64, 23, 314] });
    }

    #[test]
    fn test_solve_problem() {
        assert_eq!(MathProblem { operator: Operator::Multiply, values: vec![123, 45, 6] }.solve(), 33210);
        assert_eq!(MathProblem { operator: Operator::Add, values: vec![328, 64, 98] }.solve(), 490);
        assert_eq!(MathProblem { operator: Operator::Multiply, values: vec![51, 387, 215] }.solve(), 4243455);
        assert_eq!(MathProblem { operator: Operator::Add, values: vec![64, 23, 314] }.solve(), 401);
    }

    #[test]
    fn test_parse_input_p2() {
        let res = parse_input_p2(EXAMPLE_INPUT);

        assert!(res.is_ok(), "{:?}", res.unwrap_err());

        let problems = res.unwrap();

        assert_eq!(problems.len(), 4);
        assert_eq!(problems[0], MathProblem { operator: Operator::Multiply, values: vec![1, 24, 356] });
        assert_eq!(problems[1], MathProblem { operator: Operator::Add, values: vec![369, 248, 8] });
        assert_eq!(problems[2], MathProblem { operator: Operator::Multiply, values: vec![32, 581, 175] });
        assert_eq!(problems[3], MathProblem { operator: Operator::Add, values: vec![623, 431, 4] });
    }
}