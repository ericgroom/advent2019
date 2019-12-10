use std::collections::HashSet;

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

#[derive(PartialEq, Eq, Hash, Debug)]
enum Sign {
    Positive,
    Negative,
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

#[derive(PartialEq, Eq, Hash, Debug)]
struct Fraction {
    numerator: i32,
    denominator: i32,
    sign: Sign,
}

impl Fraction {
    fn new(numerator: i32, denominator: i32) -> Fraction {
        let mut sign = Sign::Positive;
        if numerator < 0 && denominator > 0 || denominator < 0 && numerator > 0 {
            sign = Sign::Negative;
        }
        let mut numerator = numerator.abs();
        let mut denominator = numerator.abs();
        let gcd = gcd(numerator, denominator);
        numerator /= gcd;
        denominator /= gcd;
        Fraction {
            numerator: numerator,
            denominator: denominator,
            sign: sign,
        }
    }
}

fn find_number_of_visible_asteroids(asteroid: &Point, set: &HashSet<Point>) -> usize {
    // using Sign here as direction in the x axis
    let mut slope_set: HashSet<(i32, i32, Sign)> = HashSet::new();
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
            slope_set.insert((0, 0, direction));
        } else {
            let gcd = gcd(dy, dx);
            if gcd == 0 {
                slope_set.insert((1, 0, direction));
            } else {
                slope_set.insert((dy / gcd, dx / gcd, direction));
            }
        }
    }
    slope_set.len()
}

fn find_asteroid_with_best_visibility(asteroids: HashSet<Point>) -> usize {
    let mut max_count = 0;
    for asteroid in &asteroids {
        let visibility = find_number_of_visible_asteroids(&asteroid, &asteroids);
        if visibility > max_count {
            max_count = visibility;
        }
    }
    max_count
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
    find_asteroid_with_best_visibility(input)
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
            find_number_of_visible_asteroids(&Point::new(1, 0), &points_set),
            7
        );
        assert_eq!(
            find_number_of_visible_asteroids(&Point::new(4, 0), &points_set),
            7
        );
        assert_eq!(
            find_number_of_visible_asteroids(&Point::new(0, 2), &points_set),
            6
        );
        assert_eq!(
            find_number_of_visible_asteroids(&Point::new(1, 2), &points_set),
            7
        );
        assert_eq!(
            find_number_of_visible_asteroids(&Point::new(2, 2), &points_set),
            7
        );
        assert_eq!(
            find_number_of_visible_asteroids(&Point::new(3, 2), &points_set),
            7
        );
        assert_eq!(
            find_number_of_visible_asteroids(&Point::new(4, 2), &points_set),
            5
        );
        assert_eq!(
            find_number_of_visible_asteroids(&Point::new(4, 3), &points_set),
            7
        );
        assert_eq!(
            find_number_of_visible_asteroids(&Point::new(3, 4), &points_set),
            8
        );
        assert_eq!(
            find_number_of_visible_asteroids(&Point::new(4, 4), &points_set),
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
            assert_eq!(find_asteroid_with_best_visibility(input), 8);
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
            assert_eq!(find_asteroid_with_best_visibility(input), 33);
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
            assert_eq!(find_asteroid_with_best_visibility(input), 35);
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
            assert_eq!(find_asteroid_with_best_visibility(input), 41);
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
            assert_eq!(find_asteroid_with_best_visibility(input), 210);
        }
    }

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(find_ideal_asteroid(), 334)
    }
}
