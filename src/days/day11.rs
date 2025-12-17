use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::parser::Parser;
use std::collections::{HashMap, HashSet};

pub const DAY11: Day = Day { puzzle1, puzzle2 };

fn puzzle1(input: &String) -> Result<String, String> {
    let map = parse_input(input)?;

    let result = count_data_flows(&map);

    Ok(format!("{}", result))
}
fn puzzle2(input: &String) -> Result<String, String> {
    let map = parse_input(input)?;

    let result = count_svr_flows(&map);

    Ok(format!("{}", result))
}

fn count_data_flows(map: &HashMap<String, Vec<String>>) -> usize {
    // Get the number of different paths from 'you' to 'out'
    traverse_to_out(&"you".to_string(), HashSet::new(), &vec![], map, &mut HashMap::new())
}

fn count_svr_flows(map: &HashMap<String, Vec<String>>) -> usize {
    // Get the number of different paths from 'svr' to 'out', visiting 'fft' and 'dac'
    traverse_to_out(&"svr".to_string(), HashSet::new(), &vec!["fft".to_string(), "dac".to_string()], map, &mut HashMap::new())
}

fn traverse_to_out(
    current: &String,
    path: HashSet<String>,
    must_visit: &Vec<String>,
    map: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, usize>
) -> usize {
    let cache_key = must_visit.iter().cloned().filter(|v| path.contains(v)).collect::<Vec<_>>().append_item(current).join("->");

    if let Some(&cached) = cache.get(&cache_key) {
        cached
    } else if path.contains(current) {
        0
    } else if current.eq("out") {
        if must_visit.iter().all(|v| path.contains(v)) {
            1
        } else {
            0
        }
    } else if let Some(outs) = map.get(current) {
        let mut result = 0;
        for out in outs {
            result += traverse_to_out(out, path.append_item(current), must_visit, map, cache);
        }

        cache.insert(cache_key, result);
        result
    } else {
        0
    }
}

fn parse_input(input: &str) -> Result<HashMap<String, Vec<String>>, String> {
    let mut result = HashMap::new();

    for line in input.lines() {
        let mut parser = Parser::new(line);
        let source = parser.str(3)?;
        parser.literal(":")?;

        let mut outs = vec![];
        while !parser.is_exhausted() {
            outs.push(parser.str(3)?);
        }

        result.insert(source, outs);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::days::day11::{count_data_flows, count_svr_flows, parse_input};

    const EXAMPLE_INPUT: &str = "\
        aaa: you hhh\n\
        you: bbb ccc\n\
        bbb: ddd eee\n\
        ccc: ddd eee fff\n\
        ddd: ggg\n\
        eee: out\n\
        fff: out\n\
        ggg: out\n\
        hhh: ccc fff iii\n\
        iii: out\n\
    ";

    const EXAMPLE_INPUT_P2: &str = "\
        svr: aaa bbb\n\
        aaa: fft\n\
        fft: ccc\n\
        bbb: tty\n\
        tty: ccc\n\
        ccc: ddd eee\n\
        ddd: hub\n\
        hub: fff\n\
        eee: dac\n\
        dac: fff\n\
        fff: ggg hhh\n\
        ggg: out\n\
        hhh: out\n\
    ";

    #[test]
    fn test_parse_input() {
        let res = parse_input(EXAMPLE_INPUT);
        assert!(res.is_ok());

        let map = res.unwrap();
        assert_eq!(
            map.get("you"),
            Some(&vec!["bbb".to_string(), "ccc".to_string()])
        );
    }

    #[test]
    fn test_count_data_flows() {
        let map = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(count_data_flows(&map), 5);
    }

    #[test]
    fn test_count_svr_flows() {
        let map = parse_input(EXAMPLE_INPUT_P2).unwrap();

        assert_eq!(count_svr_flows(&map), 2);
    }
}
