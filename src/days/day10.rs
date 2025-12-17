use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::parser::Parser;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

pub const DAY10: Day = Day { puzzle1, puzzle2 };

fn puzzle1(input: &String) -> Result<String, String> {
    let machines = parse_input(input)?;

    let results = machines
        .iter()
        .map(|m| {
            m.compute_least_button_presses_to_led_state()
                .ok_or(format!("No valid state found?! {:?}", m))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let result = results.iter().map(|p| p.len()).fold(0, |v, acc| acc + v);

    Ok(format!("{}", result))
}
fn puzzle2(input: &String) -> Result<String, String> {
    let machines = parse_input(input)?;

    let results = machines
        .iter()
        .map(|m| {
            m.compute_joltage_button_presses()
                .ok_or(format!("No valid state found?! {:?}", m))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let result = results.iter().fold(0, |v, acc| acc + v);

    Ok(format!("{}", result))
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Machine {
    leds: Vec<bool>,
    buttons: Vec<ButtonDefinition>,
    joltages: Vec<usize>,
}

impl Machine {
    fn compute_least_button_presses_to_led_state(&self) -> Option<Vec<ButtonDefinition>> {
        // Most likely a dumb search with a state cache should be sufficient.
        // From state A -> if pressing a button gives a new state B, store and queue.
        // Breadth first, as we need the least amount of presses to get somewhere.
        // An already seen state is by definition wrong
        let mut heap: BinaryHeap<MachineLedState> = BinaryHeap::new();
        heap.push(MachineLedState {
            leds: self.leds.iter().map(|_| false).collect(),
            button_presses: vec![],
        });

        let mut seen_states: HashSet<usize> = HashSet::new();

        while let Some(state) = heap.pop() {
            if state.is_end_state(self) {
                return Some(state.button_presses.clone());
            }

            // Not an end state, mutate by pressing buttons
            // Pressing a button already pressed cancels it out, so we can just ignore those.
            for button in self
                .buttons
                .iter()
                .filter(|b| !state.button_presses.contains(b))
            {
                let next_state = state.press_button(button);
                if seen_states.insert(next_state.get_led_value()) {
                    heap.push(next_state);
                }
            }
        }

        None
    }

    fn compute_joltage_button_presses(&self) -> Option<usize> {
        // Option 3. We leverage part 1. We can reduce our problem to odd vs even (i.e. on vs off)
        // Example: 3,5,4,7 => ##.#
        // That yields a (small) amount of possible button presses.
        // For each of the results, we'll take the remaining numbers; knowing they should be reachable by
        //  pressing some buttons an even amount of times. So we divide them by 2. (They should be even, given we reached the pattern.)
        // That gives a new pattern to solve.
        // Notes:
        // - Some patterns might not solve, we reject those
        // - Some results will get to numbers above the joltage required, we reject those as well.
        solve_joltage(
            self.joltages.clone(),
            self,
            &build_button_maps(self.leds.len(), &self.buttons),
            0,
        )
    }
}

fn get_remainder(
    pressed: &Vec<usize>,
    joltages: &Vec<usize>,
    buttons: &Vec<ButtonDefinition>,
) -> Option<Vec<usize>> {
    let mut values = joltages.clone();

    for button in pressed.iter().map(|&idx| &buttons[idx]) {
        for &wire in &button.wires {
            if values[wire] == 0 {
                return None;
            }
            values[wire] -= 1;
        }
    }

    Some(values)
}

fn solve_joltage(
    joltage: Vec<usize>,
    machine: &Machine,
    maps: &ButtonMaps,
    depth: usize,
) -> Option<usize> {
    let (joltage_map, pattern_map) = maps;

    if joltage.iter().all(|&v| v == 0) {
        return Some(0);
    } // end condition

    let pattern = joltage.iter().map(|&v| v % 2 == 1).collect::<Vec<_>>();
    let options = pattern_map.get(&pattern).cloned().unwrap_or(vec![]);

    let mut min: Option<usize> = joltage_map.get(&joltage).cloned();

    for (presses, values) in options
        .iter()
        .filter_map(|p| get_remainder(p, &joltage, &machine.buttons).map(|r| (p, r)))
    {
        if !values.iter().all(|&v| v % 2 == 0) {
            continue;
        } // invalid remainder

        // half values, solve again, add to presses * 2
        let next_joltages = values.iter().map(|v| v / 2).collect::<Vec<_>>();
        if let Some(v) = solve_joltage(next_joltages, &machine, maps, depth + 1) {
            let total_presses = 2 * v + presses.len();
            match min {
                Some(mv) if total_presses < mv => min = Some(total_presses),
                None => min = Some(total_presses),
                _ => {}
            }
        }
    }

    min
}

// 0 => []
// 1 -> 1
// 2 -> 1, 1|2, 2
// 3 => 1, 1|3, 1|2, 1|2|3, 2, 2|3, 3

fn build_button_combinations(button_count: usize) -> Vec<Vec<usize>> {
    if button_count == 0 {
        return vec![vec![]];
    }

    let button_idx = button_count - 1;
    let mut result = build_button_combinations(button_count - 1);

    // For each entry in result (from the previous amount of buttons), we add a copy that adds this button. (And also leave the original)
    result.push_all(&result.iter().map(|r| r.append_item(&button_idx)).collect());

    result
}

fn compute_joltage(
    num_wires: usize,
    buttons: &Vec<ButtonDefinition>,
    presses: &Vec<usize>,
) -> Vec<usize> {
    let mut joltage = vec![];
    for _ in 0..num_wires {
        joltage.push(0);
    }

    for &wire in presses.iter().flat_map(|&p| &buttons[p].wires) {
        joltage[wire] += 1
    }

    joltage
}

type ButtonMaps = (
    HashMap<Vec<usize>, usize>,
    HashMap<Vec<bool>, Vec<Vec<usize>>>,
);

fn build_button_maps(num_wires: usize, buttons: &Vec<ButtonDefinition>) -> ButtonMaps {
    // We build a quick look-up map, by combining all the button presses (when pressed once) to what joltage that would yield.
    // We will use this to check if there is a solution for the current joltage.
    // If not, we'll try to solve the pattern (making odd numbers even) and halving the result to try again.
    let mut joltage_map = HashMap::new();
    let mut pattern_map: HashMap<Vec<bool>, Vec<Vec<usize>>> = HashMap::new();

    for combination in build_button_combinations(buttons.len()) {
        let joltage = compute_joltage(num_wires, buttons, &combination);
        let pattern = joltage.map(|v| v % 2 == 1);
        let presses = combination.len();

        if let Some(list) = pattern_map.get_mut(&pattern) {
            list.push(combination);
        } else {
            pattern_map.insert(pattern, vec![combination]);
        }

        if let Some(&prev) = joltage_map.get(&joltage) {
            if presses < prev {
                joltage_map.insert(joltage, presses);
            }
        } else {
            joltage_map.insert(joltage, presses);
        }
    }

    (joltage_map, pattern_map)
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct ButtonDefinition {
    wires: Vec<usize>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct MachineLedState {
    leds: Vec<bool>,
    button_presses: Vec<ButtonDefinition>,
}

impl MachineLedState {
    fn is_end_state(&self, machine: &Machine) -> bool {
        if self.leds.len() != machine.leds.len() {
            false
        } else {
            self.leds
                .iter()
                .enumerate()
                .all(|(i, v)| machine.leds[i].eq(v))
        }
    }

    fn get_led_value(&self) -> usize {
        // Map leds to bits, to get a unique value of the current state.
        let mut result = 0;

        for i in 0..self.leds.len() {
            result <<= 1;
            result |= self.leds[i] as usize;
        }

        result
    }

    fn press_button(&self, button: &ButtonDefinition) -> Self {
        let mut leds = self.leds.clone();
        let mut button_presses = self.button_presses.clone();
        button_presses.push(button.clone());

        for wire in &button.wires {
            leds[*wire] ^= true;
        }

        Self {
            leds,
            button_presses,
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Machine>, String> {
    input.lines().map(|l| l.parse()).collect()
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);

        let mut leds = vec![];
        let mut buttons = vec![];
        let mut joulages = vec![];

        parser.literal("[")?;
        loop {
            match parser.one_of(vec![".", "#", "]"])? {
                "." => leds.push(false),
                "#" => leds.push(true),
                _ => break,
            }
        }

        loop {
            if parser.one_of(vec!["(", "{"])? == "{" {
                break;
            }

            let mut wires = vec![];
            loop {
                wires.push(parser.usize()?);

                if parser.one_of(vec![",", ")"])? == ")" {
                    break;
                }
            }

            buttons.push(ButtonDefinition { wires });
        }

        loop {
            joulages.push(parser.usize()?);

            if parser.one_of(vec![",", "}"])? == "}" {
                break;
            }
        }

        Ok(Machine {
            leds,
            buttons,
            joltages: joulages,
        })
    }
}

// Ensure a state with fewer presses can come out of a BinaryHeap first
impl Ord for MachineLedState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .button_presses
            .len()
            .cmp(&self.button_presses.len())
            .then_with(|| self.get_led_value().cmp(&other.get_led_value()))
    }
}

impl PartialOrd for MachineLedState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day10::{
        ButtonDefinition, Machine, MachineLedState, build_button_combinations, build_button_maps,
        parse_input, solve_joltage,
    };

    const EXAMPLE_INPUT: &str = "\
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n\
    ";

    #[test]
    fn test_parse_input() {
        let res = parse_input(EXAMPLE_INPUT);

        assert!(res.is_ok());

        let machines = res.unwrap();

        assert_eq!(
            machines[0],
            Machine {
                leds: vec![false, true, true, false],
                buttons: vec![
                    ButtonDefinition { wires: vec![3] },
                    ButtonDefinition { wires: vec![1, 3] },
                    ButtonDefinition { wires: vec![2] },
                    ButtonDefinition { wires: vec![2, 3] },
                    ButtonDefinition { wires: vec![0, 2] },
                    ButtonDefinition { wires: vec![0, 1] },
                ],
                joltages: vec![3, 5, 4, 7]
            }
        );
    }

    #[test]
    fn test_machine_state_get_led_value() {
        assert_eq!(
            MachineLedState {
                leds: vec![false, true, false, true],
                button_presses: vec![]
            }
            .get_led_value(),
            5
        );
        assert_eq!(
            MachineLedState {
                leds: vec![true, false, false, false],
                button_presses: vec![]
            }
            .get_led_value(),
            8
        );
        assert_eq!(
            MachineLedState {
                leds: vec![true, true, true, true],
                button_presses: vec![]
            }
            .get_led_value(),
            15
        );
        assert_eq!(
            MachineLedState {
                leds: vec![true, false, false, false, false, false],
                button_presses: vec![]
            }
            .get_led_value(),
            32
        );
    }

    #[test]
    fn test_machine_state_press_button() {
        let state = MachineLedState {
            leds: vec![false, false, false, false],
            button_presses: vec![],
        };

        assert_eq!(
            state.press_button(&ButtonDefinition { wires: vec![0, 3] }),
            MachineLedState {
                leds: vec![true, false, false, true],
                button_presses: vec![ButtonDefinition { wires: vec![0, 3] }]
            }
        );
    }

    #[test]
    fn test_machine_compute_least_button_presses() {
        let machines = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(
            machines[0].compute_least_button_presses_to_led_state(),
            Some(vec![
                ButtonDefinition { wires: vec![0, 1] },
                ButtonDefinition { wires: vec![0, 2] }
            ])
        );
        assert_eq!(
            machines[1]
                .compute_least_button_presses_to_led_state()
                .map(|p| p.len()),
            Some(3)
        );
        assert_eq!(
            machines[2]
                .compute_least_button_presses_to_led_state()
                .map(|p| p.len()),
            Some(2)
        );
    }

    #[test]
    fn test_machine_compute_joltage_button_presses() {
        let machines = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(machines[0].compute_joltage_button_presses(), Some(10));
        assert_eq!(machines[1].compute_joltage_button_presses(), Some(12));
        assert_eq!(machines[2].compute_joltage_button_presses(), Some(11));
    }

    #[test]
    fn test_build_button_combinations() {
        // assert_eq!(build_button_combinations(0), Vec<Vec<usize>>::new());
        assert_eq!(build_button_combinations(1), vec![vec![], vec![0]]);
        assert_eq!(
            build_button_combinations(2),
            vec![vec![], vec![0], vec![1], vec![0, 1],]
        );
        assert_eq!(
            build_button_combinations(3),
            vec![
                vec![],
                vec![0],
                vec![1],
                vec![0, 1],
                vec![2],
                vec![0, 2],
                vec![1, 2],
                vec![0, 1, 2]
            ]
        );
    }

    #[test]
    fn test_build_button_maps() {
        let (joltage, patterns) = build_button_maps(
            3,
            &vec![
                ButtonDefinition { wires: vec![0, 1] },
                ButtonDefinition { wires: vec![1, 2] },
                ButtonDefinition { wires: vec![0, 2] },
            ],
        );

        println!("{:?}", joltage);
        println!("{:?}", patterns);
        assert!(patterns.contains_key(&vec![false, false, false]));
    }

    #[test]
    fn debug_failing_case() {
        let machine = Machine {
            leds: vec![true, false, false, true, false, true, true, false],
            buttons: vec![
                ButtonDefinition { wires: vec![5, 7] },
                ButtonDefinition {
                    wires: vec![0, 5, 7],
                },
                ButtonDefinition {
                    wires: vec![0, 1, 3, 4, 5, 6, 7],
                },
                ButtonDefinition { wires: vec![0, 1] },
                ButtonDefinition {
                    wires: vec![1, 4, 6],
                },
                ButtonDefinition {
                    wires: vec![0, 1, 2, 4, 6, 7],
                },
                ButtonDefinition {
                    wires: vec![0, 2, 5],
                },
                ButtonDefinition {
                    wires: vec![2, 3, 4],
                },
            ],
            joltages: vec![37, 32, 27, 22, 44, 40, 27, 36],
        };

        // println!("{:?}", machine.compute_joltage_button_presses2());

        assert_eq!(
            solve_joltage(
                machine.joltages.clone(),
                &machine,
                &build_button_maps(machine.leds.len(), &machine.buttons),
                0
            ),
            Some(8 + 4 + 18 + 2 + 16 + 10 + 16 + 6 + 4)
        );
    }
}
