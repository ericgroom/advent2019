use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn slope(&self, other: Point) -> (i32, i32) {
        (self.y - other.y, self.x - other.x)
    }

    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd)]
enum Sign {
    Positive,
    Negative,
}

impl Ord for Sign {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Positive, Self::Negative) => Ordering::Greater,
            (Self::Negative, Self::Positive) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x.abs();
    let mut y = y.abs();
    if x == 0 || y == 0 {
        return 0;
    }
    while x != y {
        if x > y {
            x = x - y;
        } else {
            y = y - x;
        }
    }

    x
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd)]
struct SlopeInfo {
    dy: i32,
    dx: i32,
    x_direction: Sign,
}

impl SlopeInfo {
    fn new(dy: i32, dx: i32, x_direction: Sign) -> SlopeInfo {
        SlopeInfo {
            dy: dy,
            dx: dx,
            x_direction: x_direction,
        }
    }
}

impl Ord for SlopeInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        // self is greater than other if
        // self is a vertical line
        // self has a steeper slope than other
        // if other is on the same slope, differentite by x_direction
        if self.dx == 0 {
            if other.dy == 0 {
                return self.x_direction.cmp(&other.x_direction);
            } else {
                return Ordering::Greater;
            }
        }
        let self_slope = self.dy / self.dy;
        let other_slope = other.dy as f64 / other.dx as f64;
        if self.dy == 0 && other.dy == 0 {
            return self.x_direction.cmp(&other.x_direction);
        }
        Ordering::Equal
        // special cases
        // 1. equal
        // 2. horizontal line
        // 3. vertical line
        // Also need to enforce clockwise direction
        // if self.dx == other.dx && self.dy == other.dy {
        //     return self.x_direction.cmp(&other.x_direction);
        // } else if self.dx == other.dx {
        //     return self.dy.cmp(&other.dy);
        // }
    }
}

fn find_visible_asteroids(asteroid: &Point, set: &HashSet<Point>) -> HashMap<SlopeInfo, Point> {
    // using Sign here as direction in the x axis
    let mut slope_map: HashMap<SlopeInfo, Point> = HashMap::new();
    for candidate in set {
        if asteroid.x == candidate.x && asteroid.y == candidate.y {
            continue;
        }
        let (dy, dx) = asteroid.slope(*candidate);
        let direction = if asteroid.x > candidate.x {
            Sign::Negative
        } else if asteroid.x == candidate.x {
            if asteroid.y > candidate.y {
                Sign::Positive
            } else if asteroid.y == candidate.y {
                panic!("same asteroid")
            } else {
                Sign::Negative
            }
        } else {
            Sign::Positive
        };
        if dx == 0 {
            slope_map.insert(SlopeInfo::new(0, 0, direction), *candidate);
        } else {
            let gcd = gcd(dy, dx);
            if gcd == 0 {
                slope_map.insert(SlopeInfo::new(1, 0, direction), *candidate);
            } else {
                slope_map.insert(SlopeInfo::new(dy / gcd, dx / gcd, direction), *candidate);
            }
        }
    }
    slope_map
}

fn find_asteroid_with_best_visibility(asteroids: &HashSet<Point>) -> (Point, usize) {
    let mut max_count = 0;
    let mut max_asteroid = None;
    for asteroid in asteroids {
        let visibility = find_visible_asteroids(&asteroid, &asteroids).len();
        if visibility > max_count {
            max_count = visibility;
            max_asteroid = Some(asteroid);
        }
    }
    (*max_asteroid.unwrap(), max_count)
}

fn read_input(s: &str) -> HashSet<Point> {
    let mut result = HashSet::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                result.insert(Point::new(x as i32, y as i32));
            }
        }
    }
    result
}

fn get_test_input() -> HashSet<Point> {
    read_input(include_str!("day10_input.txt"))
}

pub fn find_ideal_asteroid() -> usize {
    let input = get_test_input();
    let (_, visibility) = find_asteroid_with_best_visibility(&input);
    visibility
}

// find center node
// find all visible nodes from that node
// order ascending by slope and x direction
// destroy nodes in that order
// goto 2 with new set

fn sort_visible_asteroids_by_slope(
    central_asteroid: &Point,
    others: &HashMap<SlopeInfo, Point>,
) -> Vec<Point> {
    vec![]
}

fn get_destruction_order(asteroids: HashSet<Point>) -> Vec<Point> {
    let result = Vec::with_capacity(asteroids.len());
    let mut asteroids = asteroids;
    let (central_asteroid, _) = find_asteroid_with_best_visibility(&asteroids);
    while !asteroids.is_empty() {
        let visible_asteroids = find_visible_asteroids(&central_asteroid, &asteroids);
        let sorted_in_order_of_destruction =
            sort_visible_asteroids_by_slope(&central_asteroid, &visible_asteroids);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slope() {
        let (dy, dx) = Point::new(4, 0).slope(Point::new(3, 4));
        assert_eq!(dy as f64 / dx as f64, -4.0);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(4, 2), 2);
        assert_eq!(gcd(-2, -4), 2);
        assert_eq!(gcd(-4, 2), 2);
    }

    #[test]
    fn test_find_number_of_visible_asteroids() {
        let mut points = vec![
            Point::new(1, 0),
            Point::new(4, 0),
            Point::new(0, 2),
            Point::new(1, 2),
            Point::new(2, 2),
            Point::new(3, 2),
            Point::new(4, 2),
            Point::new(4, 3),
            Point::new(3, 4),
            Point::new(4, 4),
        ];
        let mut points_set = HashSet::new();
        for point in points.drain(..) {
            points_set.insert(point);
        }

        assert_eq!(
            find_visible_asteroids(&Point::new(1, 0), &points_set).len(),
            7
        );
        assert_eq!(
            find_visible_asteroids(&Point::new(4, 0), &points_set).len(),
            7
        );
        assert_eq!(
            find_visible_asteroids(&Point::new(0, 2), &points_set).len(),
            6
        );
        assert_eq!(
            find_visible_asteroids(&Point::new(1, 2), &points_set).len(),
            7
        );
        assert_eq!(
            find_visible_asteroids(&Point::new(2, 2), &points_set).len(),
            7
        );
        assert_eq!(
            find_visible_asteroids(&Point::new(3, 2), &points_set).len(),
            7
        );
        assert_eq!(
            find_visible_asteroids(&Point::new(4, 2), &points_set).len(),
            5
        );
        assert_eq!(
            find_visible_asteroids(&Point::new(4, 3), &points_set).len(),
            7
        );
        assert_eq!(
            find_visible_asteroids(&Point::new(3, 4), &points_set).len(),
            8
        );
        assert_eq!(
            find_visible_asteroids(&Point::new(4, 4), &points_set).len(),
            7
        );
    }

    #[test]
    fn test_read_input() {
        let expected = {
            let mut points = vec![
                Point::new(1, 0),
                Point::new(4, 0),
                Point::new(0, 2),
                Point::new(1, 2),
                Point::new(2, 2),
                Point::new(3, 2),
                Point::new(4, 2),
                Point::new(4, 3),
                Point::new(3, 4),
                Point::new(4, 4),
            ];
            let mut points_set = HashSet::new();
            for point in points.drain(..) {
                points_set.insert(point);
            }
            points_set
        };
        let actual = read_input(".#..#\n.....\n#####\n....#\n...##");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_asteroid_with_best_visibility() {
        {
            let input = read_input(".#..#\n.....\n#####\n....#\n...##");
            assert_eq!(find_asteroid_with_best_visibility(&input).1, 8);
        }
        {
            let input = read_input(
                "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
            );
            assert_eq!(find_asteroid_with_best_visibility(&input).1, 33);
        }
        {
            let input = read_input(
                "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.",
            );
            assert_eq!(find_asteroid_with_best_visibility(&input).1, 35);
        }
        {
            let input = read_input(
                ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..",
            );
            assert_eq!(find_asteroid_with_best_visibility(&input).1, 41);
        }
        {
            let input = read_input(
                ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
            );
            assert_eq!(find_asteroid_with_best_visibility(&input).1, 210);
        }
    }

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(find_ideal_asteroid(), 334)
    }
}
