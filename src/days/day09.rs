use crate::days::Day;
use crate::util::geometry::{Point};

pub const DAY9: Day = Day { puzzle1, puzzle2 };

fn puzzle1(input: &String) -> Result<String, String> {
    let points = parse_input(input)?;

    let rect = find_largest_rectangle(&points).ok_or("No rectangle found".to_string())?;

    Ok(format!("{}", rect.area()))
}
fn puzzle2(input: &String) -> Result<String, String> {
    let points = parse_input(input)?;

    let rect = find_largest_rectangle_in_path(&points).ok_or("No rectangle found".to_string())?;

    Ok(format!("{}", rect.area()))
}

fn parse_input(input: &str) -> Result<Vec<Point>, String> {
    input.lines().map(|l| l.parse()).collect()
}

fn find_largest_rectangle(points: &Vec<Point>) -> Option<Rectangle> {
    // Find the two points with the largest rectangle area between them.
    let mut result: Option<Rectangle> = None;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let rect = Rectangle {
                corners: [points[i], points[j]],
            };
            if let Some(ref current) = result {
                if current.area() < rect.area() {
                    result = Some(rect)
                }
            } else {
                result = Some(rect)
            }
        }
    }

    result
}

fn find_largest_rectangle_in_path(path: &Vec<Point>) -> Option<Rectangle> {
    // The list of points describe a path, any tile on the path and inside the path is green
    // The points in the list are still red, as before.
    // The largest rectangle still needs points from path, but can only include points that are red or green.

    let mut lines = vec![];
    for i in 0..path.len() {
        let p1 = path[i];
        let p2 = path[(i+1) % path.len()];

        lines.push([p1,p2]);
    }

    let mut rects = vec![];

    for i in 0..path.len() {
        for j in (i + 1)..path.len() {
            let rect = Rectangle {
                corners: [path[i], path[j]],
            };

            rects.push(rect);
        }
    }

    rects.sort_by_key(|r| r.area());
    rects.reverse();

    rects.iter().find(|r| {
        let [p1, p2] = r.corners;

        let xmin = p1.x.min(p2.x);
        let xmax = p1.x.max(p2.x);
        let ymin = p1.y.min(p2.y);
        let ymax = p1.y.max(p2.y);

        // The rectangle is within the polygon if all the line segments are actually around the rectangle
        // i.e. fully above, below, left, or right of it.
        lines.iter().all(|&[Point { x: px, y: py }, Point { x: qx, y: qy }]| {
            if py == qy {
                // Horizontal
                py >= ymax
                    || py <= ymin
                    || (px <= xmin && qx <= xmin)
                    || (px >= xmax && qx >= xmax)
            } else {
                // Vertical
                px >= xmax
                    || px <= xmin
                    || (py <= ymin && qy <= ymin)
                    || (py >= ymax && qy >= ymax)
            }
        })
    }).copied()
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Rectangle {
    corners: [Point; 2],
}

impl Rectangle {
    fn area(&self) -> isize {
        let [p1, p2] = self.corners;

        ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day09::{Rectangle, find_largest_rectangle, parse_input, find_largest_rectangle_in_path};

    const EXAMPLE_INPUT: &str = "\
        7,1\n\
        11,1\n\
        11,7\n\
        9,7\n\
        9,5\n\
        2,5\n\
        2,3\n\
        7,3\n\
    ";

    #[test]
    fn test_parse_input() {
        let res = parse_input(EXAMPLE_INPUT);

        assert!(res.is_ok());

        let points = res.unwrap();

        assert_eq!(points.len(), 8);
    }

    #[test]
    fn test_find_largest_rectangle() {
        let points = parse_input(EXAMPLE_INPUT).unwrap();

        let res = find_largest_rectangle(&points);

        assert_eq!(
            res,
            Some(Rectangle {
                corners: [(11, 1).into(), (2, 5).into()]
            })
        );
        assert_eq!(res.map(|r| r.area()), Some(50));
    }

    #[test]
    fn test_find_largest_rectangle_in_path() {
        let points = parse_input(EXAMPLE_INPUT).unwrap();

        let res = find_largest_rectangle_in_path(&points);

        assert_eq!(
            res,
            Some(Rectangle {
                corners: [(9, 5).into(), (2, 3).into()]
            })
        );
        assert_eq!(res.map(|r| r.area()), Some(24));
    }
}
