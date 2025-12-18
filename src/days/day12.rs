use std::collections::HashSet;
use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::geometry::{Bounds, Grid, Point};
use crate::util::parser::Parser;

pub const DAY12: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) -> Result<String, String> {
    let puzzle = parse_input(input)?;

    let mut solvable = 0;
    for i in 0..puzzle.areas.len() {
        println!("Solving {}", i);
        if let Some(_) = puzzle.solve_area(i) {
            solvable += 1;
        }
    }

    Ok(solvable.to_string())
}
fn puzzle2(_input: &String) -> Result<String, String> {
    Ok("Last puzzle be freebie!".to_string())
}

type Shape = [[bool; 3]; 3];

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Area {
    width: usize,
    height: usize,
    presents: [usize; 6]
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Puzzle {
    presents: [Shape; 6],
    areas: Vec<Area>,
}

impl Puzzle {
    fn solve_area(&self, area_idx: usize) -> Option<Grid<bool>> {
        let area = self.areas[area_idx];
        let initial_state = AreaState::new(&area);

        let available = area.width * area.height;
        // Dumb filter; if the presents fit tiled (just next to each other), no need to compute
        let tiled: usize = area.presents.iter().copied().sum();
        if (tiled * 9) <= available {
            println!("Fits easily (just tile everything without compacting)");
            return Some(Grid::empty()) // We don't, at this point, use the grid, so we don't fill it.
        }

        // Dumb filter; if the amount of tiles to place is more than the area, it will definitely never fit.
        let needed: usize = area.presents.iter().enumerate()
            .map(|(i, &v)| if v == 0 { 0 } else { Vec::from(self.presents[i]).iter().flat_map(|v| Vec::from(v)).filter(|&v| v).count() * v })
            .sum();

        if needed > available {
            println!("Area can never fit all presents :silly:");
            return None
        }

        // How do we try fit all required presents on the area?
        // Our real data has areas up to 50x50; which yields 17.672 (47x47x4x2) possible places for the first piece
        // That gives little hope for a brute force; although the amount of positions for next pieces might be significantly less...
        // Since most likely the space has to be filled quite well, maybe we can tetris tiles in:
        // 1. First goes top-left, in all 4 rotations as well as mirrored (if different) (= 8 options, times 6 possible first shapes, 42 states)
        // 2. Second is tetrised against the first, with 0, 1, or 2 px offset, moving left and up, in all 4 rotations and mirror (if different) (= 6 * 8 options, times 6 shapes, 384 states)
        // 3. Third is once again tetrised, with offsets based on the max X/Y coordinates of the current shape)
        // Etc.

        let mut seen: HashSet<String> = HashSet::new();

        fn try_solve(state: &AreaState, seen: &mut HashSet<String>, puzzle: &Puzzle) -> Option<Grid<bool>> {
            // println!("{:?}", state.shapes_to_place);

            if state.shapes_to_place.iter().all(|&v| v == 0) { return Some(state.area.clone()) }

            for shape_idx in 0..6 {
                let new_states = state.try_place_shape(shape_idx, puzzle);

                for new_state in new_states {
                    if seen.insert(new_state.to_key()) {
                        if let Some(v) = try_solve(&new_state, seen, puzzle) {
                            return Some(v)
                        }
                    }
                }
            }

            None
        }

        try_solve(&initial_state, &mut seen, self)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct AreaState {
    width: usize,
    height: usize,
    shapes_to_place: [usize; 6],
    area: Grid<bool>
}

impl AreaState {
    fn new(area: &Area) -> AreaState {
        AreaState {
            width: area.width,
            height: area.height,
            shapes_to_place: area.presents,
            area: Grid::with_size(Bounds::from_size(1, 1)),
        }
    }

    fn to_key(&self) -> String {
        // In order to be able to cache if we've already seen a state, we key the state in a way that
        // we know if two states are the same.

        self.shapes_to_place.map(|v| v.to_string()).join(",") + "|" + &self.area.values().map(|&v| if v { "#" } else { "." }).join("")
    }

    fn try_place_shape(&self, shape_idx: usize, puzzle: &Puzzle) -> Vec<AreaState> {
        if self.shapes_to_place[shape_idx] == 0 { return vec![] }

        // Try fit the shape in all possible places & orientations around the current filled grid
        let mut new_shapes_to_place = self.shapes_to_place.clone();
        new_shapes_to_place[shape_idx] -= 1;
        let orientations = get_shape_orientations(&puzzle.presents[shape_idx]);

        self.area.bounds.points().iter()
            .flat_map(|p| orientations.iter().filter_map(|s| self.try_place_shape_at(s, p)))
            .map(|area| AreaState { width: self.width, height: self.height, shapes_to_place: new_shapes_to_place, area })
            .collect()
    }

    fn try_place_shape_at(&self, shape: &Shape, point: &Point) -> Option<Grid<bool>> {
        // See if the given shape fits at x,y
        let mut result = self.area.clone();

        for py in 0..3 {
            for px in 0..3 {
                let point = (point.x + px, point.y + py).into();

                let current = result.get(&point).unwrap_or(false);
                let target = shape[py as usize][px as usize];

                if current && target { return None } // overlaps
                result.set(point, current || target);
            }
        }

        if result.bounds.width > self.width || result.bounds.height > self.height { return None } // out of bounds

        Some(result)
    }
}

fn get_shape_orientations(shape: &Shape) -> Vec<Shape> {
    let mut result = vec![];

    for mirror in [false, true] {
        for rotate in 0..4 {
            let rotated = rotate_shape(shape, rotate);
            let transformed = if mirror { mirror_shape(&rotated) } else { rotated };
            if !result.contains(&transformed) {
                result.push(transformed);
            }
        }
    }

    result
}

fn mirror_shape(shape: &Shape) -> Shape {
    // y -> y
    // x -> 3 - x
    translate_shape(shape, |x, _| 2- x, |_, y| y)
}

fn rotate_shape(shape: &Shape, steps: usize) -> Shape {
    // 1 (90deg) { x -> y, y -> 2-x }
    // 2 (180deg) { x -> 2-x, y -> 2-y }
    // 3 (270deg) { x -> 2-y, y -> x}
    match steps {
        0 => translate_shape(shape, |x,_| x, |_, y| y),
        1 => translate_shape(shape, |_,y| y, |x,_| 2-x),
        2 => translate_shape(shape, |x,_| 2-x, |_,y| 2-y),
        3 => translate_shape(shape, |_,y| 2-y, |x,_| x),
        _ => panic!("Invalid argument, steps should be 1, 2, or 3.")
    }
}

fn translate_shape<CX, CY>(shape: &Shape, cx: CX, cy: CY) -> Shape
    where CX: Fn(usize, usize) -> usize, CY: Fn(usize, usize) -> usize {
    let mut result: Shape = Default::default();

    for y in 0..3 {
        for x in 0..3 {
            result[y][x] = shape[cy(x, y)][cx(x, y)];
        }
    }

    result
}

fn parse_input(input: &str) -> Result<Puzzle, String> {
    let lines = input.lines().collect::<Vec<_>>();
    let sections = lines.split(|l| l.is_empty()).map(|l| l.iter().cloned().collect::<Vec<_>>()).collect::<Vec<_>>();

    let Some((areas_input, presents_input)) = sections.split_last() else { return Err("Could not successfully split input".to_string()) };
    let presents = parse_presents(presents_input)?;
    let areas = parse_areas(areas_input)?;

    Ok(Puzzle { presents, areas })
}

fn parse_presents(inputs: &[Vec<&str>]) -> Result<[Shape; 6], String> {
    if inputs.len() != 6 { return Err(format!("Invalid amount of presents: {}", inputs.len())) }

    let mut presents: [Shape; 6] = Default::default();

    for i in 0..6 {
        presents[i] = parse_shape(&inputs[i])?;
    }

    Ok(presents)
}

fn parse_areas(input: &Vec<&str>) -> Result<Vec<Area>, String> {
    input.iter().map(|line| {
        let mut parser = Parser::new(line);
        let width = parser.usize()?;
        parser.literal("x")?;
        let height = parser.usize()?;
        parser.literal(":")?;

        let mut presents: [usize; 6] = Default::default();
        for i in 0..6 {
            presents[i] = parser.usize()?;
        }

        Ok(Area { width, height, presents })
    }).collect()
}

fn parse_shape(input: &Vec<&str>) -> Result<Shape, String> {
    // Shape is 3 by 3, consisting of # and .; with a leading index number we'll ignore.
    let mut result: [[bool; 3]; 3] = Default::default();
    for y in 0..3 {
        for x in 0..3 {
            match input[y+1].chars().nth(x) {
                Some('#') => result[y][x] = true,
                Some('.') => result[y][x] = false,
                Some(c) => return Err(format!("Invalid character in shape: {}", c)),
                None => return Err(format!("Missing character in shape: {},{}", x, y)),
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::days::day12::{get_shape_orientations, mirror_shape, parse_input, rotate_shape, Area, AreaState, Shape};
    use crate::util::geometry::{Bounds, Grid};

    const EXAMPLE_INPUT: &str = "\
        0:\n\
        ###\n\
        ##.\n\
        ##.\n\
        \n\
        1:\n\
        ###\n\
        ##.\n\
        .##\n\
        \n\
        2:\n\
        .##\n\
        ###\n\
        ##.\n\
        \n\
        3:\n\
        ##.\n\
        ###\n\
        ##.\n\
        \n\
        4:\n\
        ###\n\
        #..\n\
        ###\n\
        \n\
        5:\n\
        ###\n\
        .#.\n\
        ###\n\
        \n\
        4x4: 0 0 0 0 2 0\n\
        12x5: 1 0 1 0 2 2\n\
        12x5: 1 0 1 0 3 2\n\
    ";

    #[test]
    fn test_parse_input() {
        let res = parse_input(EXAMPLE_INPUT);
        assert!(res.is_ok(), "Expected OK result, got: {}", res.unwrap_err());

        let puzzle = res.unwrap();
        assert_eq!(puzzle.presents[0], [[true, true, true], [true, true, false], [true, true, false]]);
        assert_eq!(puzzle.presents[1], [[true, true, true], [true, true, false], [false, true, true]]);

        assert_eq!(puzzle.areas[0], Area { width: 4, height: 4, presents: [0, 0, 0, 0, 2, 0] });
    }

    #[test]
    fn test_rotate_shape() {
        let shape: Shape = [[true, true, false], [false, true, false], [false, true, true]];

        assert_eq!(rotate_shape(&shape, 1), [[false, false, true], [true, true, true], [true, false, false]]);
        assert_eq!(rotate_shape(&shape, 2), [[true, true, false], [false, true, false], [false, true, true]]);
        assert_eq!(rotate_shape(&shape, 3), [[false, false, true], [true, true, true], [true, false, false]]);
    }

    #[test]
    fn test_mirror_shape() {
        let shape: Shape = [[true, true, false], [false, true, false], [false, true, true]];

        assert_eq!(mirror_shape(&shape), [[false, true, true], [false, true, false], [true, true, false]]);
    }

    #[test]
    fn test_get_shape_orientations() {
        let shape: Shape = [[true, true, false], [false, true, false], [false, true, true]];

        // Expected results:
        // ##. .## #.. ..#
        // .#. .#. ### ###
        // .## ##. ..# #..
        let mutations = get_shape_orientations(&shape);
        assert_eq!(mutations.len(), 4); // Only 4 unique mutations for this shape
        assert!(mutations.contains(&[[true, true, false], [false, true, false], [false, true, true]]));
        assert!(mutations.contains(&[[false, true, true], [false, true, false], [true, true, false]]));
        assert!(mutations.contains(&[[true, false, false], [true, true, true], [false, false, true]]));
        assert!(mutations.contains(&[[false, false, true], [true, true, true], [true, false, false]]));
    }

    #[test]
    fn test_area_state_try_place_shape() {
        let puzzle = parse_input(EXAMPLE_INPUT).unwrap();
        let state = AreaState {
            width: 4,
            height: 4,
            shapes_to_place: [0, 0, 0, 0, 2, 0],
            area: Grid::with_size(Bounds::from_size(1, 1))
        };

        let new_states = state.try_place_shape(4, &puzzle);
        assert_eq!(new_states.len(), 4);
        assert_eq!(grid_to_string(&new_states[0].area), "\
            ###\n\
            #..\n\
            ###\
        ");

        assert_eq!(grid_to_string(&new_states[1].area), "\
            ###\n\
            #.#\n\
            #.#\
        ");

        assert_eq!(grid_to_string(&new_states[2].area), "\
            ###\n\
            ..#\n\
            ###\
        ");


        assert_eq!(grid_to_string(&new_states[3].area), "\
            #.#\n\
            #.#\n\
            ###\
        ");

        let new_states_0 = new_states[0].try_place_shape(4, &puzzle);
        // There is only one way the desired shape fits with this state
        assert_eq!(new_states_0.len(), 1);
        assert_eq!(grid_to_string(&new_states_0[0].area), "\
            ###.\n\
            ####\n\
            ####\n\
            .###\
        ");

        let new_states_1 = new_states[1].try_place_shape(4, &puzzle);
        assert_eq!(new_states_1.len(), 1);
        assert_eq!(grid_to_string(&new_states_1[0].area), "\
            ###.\n\
            ####\n\
            ####\n\
            .###\
        "); // Worthwhile note: same resulting shape as the other option!

        assert_eq!(new_states[2].try_place_shape(4, &puzzle), vec![]); // no valid options
        assert_eq!(new_states[3].try_place_shape(4, &puzzle), vec![]); // no valid options
    }

    #[test]
    fn test_puzzle_solve_area() {
        let puzzle = parse_input(EXAMPLE_INPUT).unwrap();

        let result = puzzle.solve_area(1);
        assert!(result.is_some());

        // AAA | .BB | CCC | DDD
        // AA. | BBB | C.. | .D.
        // AA. | BB. | CCC | DDD
        // 1   | 1   | 2   | 2
        //
        // DDD...BB.CCC
        // .Dddd.BBBC.C
        // DDDdCCCBBCAC
        // ..ddd.C.AAA.
        // ....CCC.AAA.
        assert_eq!(grid_to_string(&result.unwrap()), "\
            ###...##.###\n\
            .####.####.#\n\
            ############\n\
            ..###.#.###.\n\
            ....###.###.\
        ");

        assert_eq!(puzzle.solve_area(2), None);
    }

    fn grid_to_string(grid: &Grid<bool>) -> String {
        let mut result: String = Default::default();

        for y in grid.bounds.y() {
            if !result.is_empty() {
                result = result + "\n";
            }

            for x in grid.bounds.x() {
                if grid.get(&(x, y).into()) == Some(true) {
                    result = result + "#";
                } else {
                    result = result + ".";
                }
            }
        }

        result
    }
}