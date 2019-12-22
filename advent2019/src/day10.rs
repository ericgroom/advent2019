use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use crate::utils::geometry::Vec2D;
use crate::utils::math::gcd;

impl Vec2D {
    fn slope(&self, other: Vec2D) -> (i32, i32) {
        // negate because y-axis is inverted
        (-(self.y - other.y), self.x - other.x)
    }

    fn x_direction(&self, other: &Self) -> Sign {
        if self.x > other.x {
            Sign::Negative
        } else if self.x == other.x {
            if self.y > other.y {
                Sign::Positive
            } else if self.y == other.y {
                panic!("same asteroid")
            } else {
                Sign::Negative
            }
        } else {
            Sign::Positive
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Copy, Clone)]
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
        match self.x_direction.cmp(&other.x_direction) {
            Ordering::Equal => {
                if self.dx == other.dx && self.dy == other.dy {
                    panic!("This should never happen as they wouldn't have the same visibility")
                }
                if self.dx == 0 && self.dy == 1 && other.dx == 1 && other.dy == 0 {
                    return Ordering::Greater;
                }
                let self_slope = self.dy as f64 / self.dx as f64;
                let other_slope = other.dy as f64 / other.dx as f64;
                let epsilon = 0.001;
                if (self_slope - other_slope).abs() < epsilon {
                    Ordering::Equal
                } else if self_slope > other_slope {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            ordering => ordering,
        }
    }
}

fn find_visible_asteroids(asteroid: &Vec2D, set: &HashSet<Vec2D>) -> HashMap<SlopeInfo, Vec2D> {
    // using Sign here as direction in the x axis
    let mut slope_map: HashMap<SlopeInfo, Vec2D> = HashMap::new();
    for candidate in set {
        if asteroid.x == candidate.x && asteroid.y == candidate.y {
            continue;
        }
        let (dy, dx) = asteroid.slope(*candidate);
        let direction = asteroid.x_direction(&candidate);
        let key = {
            if dx == 0 {
                SlopeInfo::new(1, 0, direction)
            } else if dy == 0 {
                SlopeInfo::new(0, 1, direction)
            } else {
                let multiplier = if (dy < 0) && (dx < 0) { -1 } else { 1 };
                let gcd = gcd(dy as i64, dx as i64) as i32;

                SlopeInfo::new(dy / gcd * multiplier, dx / gcd * multiplier, direction)
            }
        };
        let should_replace_key = match slope_map.get(&key) {
            Some(point) => asteroid.euclidean(candidate) < asteroid.euclidean(point),
            None => true,
        };
        if should_replace_key {
            slope_map.insert(key, *candidate);
        }
    }
    slope_map
}

fn find_asteroid_with_best_visibility(asteroids: &HashSet<Vec2D>) -> (Vec2D, usize) {
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

fn read_input(s: &str) -> HashSet<Vec2D> {
    let mut result = HashSet::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                result.insert(Vec2D::new(x as i32, y as i32));
            }
        }
    }
    result
}

fn get_test_input() -> HashSet<Vec2D> {
    read_input(include_str!("day10_input.txt"))
}

pub fn find_ideal_asteroid() -> usize {
    let input = get_test_input();
    let (_, visibility) = find_asteroid_with_best_visibility(&input);
    visibility
}

fn sort_visible_asteroids_by_slope(others: &mut HashMap<SlopeInfo, Vec2D>) -> Vec<Vec2D> {
    let mut result: Vec<_> = others.drain().collect();
    result.sort_by(|a, b| a.0.cmp(&b.0).reverse()); // sort decending by SlopeInfo
    result.drain(..).map(|entry| entry.1).collect()
}

fn get_destruction_order(asteroids: HashSet<Vec2D>) -> Vec<Vec2D> {
    let mut result = Vec::with_capacity(asteroids.len());
    let mut asteroids = asteroids;
    let (central_asteroid, _) = find_asteroid_with_best_visibility(&asteroids);
    asteroids.remove(&central_asteroid);
    while !asteroids.is_empty() {
        let mut visible_asteroids = find_visible_asteroids(&central_asteroid, &asteroids);
        let sorted_in_order_of_destruction =
            sort_visible_asteroids_by_slope(&mut visible_asteroids);
        for asteroid in sorted_in_order_of_destruction {
            asteroids.remove(&asteroid);
            result.push(asteroid);
        }
    }
    result
}

pub fn get_two_hundredth() -> i32 {
    let input = get_test_input();
    let ordered = get_destruction_order(input);
    let result = ordered[199];
    result.x * 100 + result.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slope() {
        let (dy, dx) = Vec2D::new(4, 0).slope(Vec2D::new(3, 4));
        assert_eq!(dy as f64 / dx as f64, 4.0);
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
            Vec2D::new(1, 0),
            Vec2D::new(4, 0),
            Vec2D::new(0, 2),
            Vec2D::new(1, 2),
            Vec2D::new(2, 2),
            Vec2D::new(3, 2),
            Vec2D::new(4, 2),
            Vec2D::new(4, 3),
            Vec2D::new(3, 4),
            Vec2D::new(4, 4),
        ];
        let mut points_set = HashSet::new();
        for point in points.drain(..) {
            points_set.insert(point);
        }

        assert_eq!(
            find_visible_asteroids(&Vec2D::new(1, 0), &points_set).len(),
            7
        );
        assert_eq!(
            find_visible_asteroids(&Vec2D::new(4, 0), &points_set).len(),
            7
        );
        assert_eq!(
            find_visible_asteroids(&Vec2D::new(0, 2), &points_set).len(),
            6
        );
        assert_eq!(
            find_visible_asteroids(&Vec2D::new(1, 2), &points_set).len(),
            7
        );
        assert_eq!(
            find_visible_asteroids(&Vec2D::new(2, 2), &points_set).len(),
            7
        );
        assert_eq!(
            find_visible_asteroids(&Vec2D::new(3, 2), &points_set).len(),
            7
        );
        assert_eq!(
            find_visible_asteroids(&Vec2D::new(4, 2), &points_set).len(),
            5
        );
        assert_eq!(
            find_visible_asteroids(&Vec2D::new(4, 3), &points_set).len(),
            7
        );
        assert_eq!(
            find_visible_asteroids(&Vec2D::new(3, 4), &points_set).len(),
            8
        );
        assert_eq!(
            find_visible_asteroids(&Vec2D::new(4, 4), &points_set).len(),
            7
        );
    }

    #[test]
    fn test_read_input() {
        let expected = {
            let mut points = vec![
                Vec2D::new(1, 0),
                Vec2D::new(4, 0),
                Vec2D::new(0, 2),
                Vec2D::new(1, 2),
                Vec2D::new(2, 2),
                Vec2D::new(3, 2),
                Vec2D::new(4, 2),
                Vec2D::new(4, 3),
                Vec2D::new(3, 4),
                Vec2D::new(4, 4),
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

    #[test]
    fn test_destruction_order() {
        let input = read_input(
            ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##",
        );
        let destruction_order = get_destruction_order(input);
        assert_eq!(
            destruction_order.into_iter().collect::<Vec<_>>(),
            vec![
                Vec2D::new(8, 1),
                Vec2D::new(9, 0),
                Vec2D::new(9, 1),
                Vec2D::new(10, 0),
                Vec2D::new(9, 2),
                Vec2D::new(11, 1),
                Vec2D::new(12, 1),
                Vec2D::new(11, 2),
                Vec2D::new(15, 1),
                Vec2D::new(12, 2),
                Vec2D::new(13, 2),
                Vec2D::new(14, 2),
                Vec2D::new(15, 2),
                Vec2D::new(12, 3),
                Vec2D::new(16, 4),
                Vec2D::new(15, 4),
                Vec2D::new(10, 4),
                Vec2D::new(4, 4),
                Vec2D::new(2, 4),
                Vec2D::new(2, 3),
                Vec2D::new(0, 2),
                Vec2D::new(1, 2),
                Vec2D::new(0, 1),
                Vec2D::new(1, 1),
                Vec2D::new(5, 2),
                Vec2D::new(1, 0),
                Vec2D::new(5, 1),
                Vec2D::new(6, 1),
                Vec2D::new(6, 0),
                Vec2D::new(7, 0),
                Vec2D::new(8, 0),
                Vec2D::new(10, 1),
                Vec2D::new(14, 0),
                Vec2D::new(16, 1),
                Vec2D::new(13, 3),
                Vec2D::new(14, 3),
            ]
        )
    }

    #[test]
    fn test_slope_info_order() {
        {
            let vertical_line = SlopeInfo::new(4, 0, Sign::Positive);
            let positive_slope = SlopeInfo::new(2, 1, Sign::Positive);
            assert_eq!(vertical_line.cmp(&positive_slope), Ordering::Greater);
        }
        {
            let vertical_line_pos = SlopeInfo::new(4, 0, Sign::Positive);
            let vertical_line_neg = SlopeInfo::new(4, 0, Sign::Negative);
            assert_eq!(vertical_line_pos.cmp(&vertical_line_neg), Ordering::Greater);
        }
        {
            let horizontal_line_pos = SlopeInfo::new(0, 4, Sign::Positive);
            let neg_slope = SlopeInfo::new(1, -2, Sign::Negative);
            assert_eq!(horizontal_line_pos.cmp(&neg_slope), Ordering::Greater);
        }
        {
            let horizontal_line_pos = SlopeInfo::new(0, 1, Sign::Positive);
            let vertical_line_pos = SlopeInfo::new(1, 0, Sign::Positive);
            assert_eq!(horizontal_line_pos.cmp(&vertical_line_pos), Ordering::Less);
        }
    }

    #[test]
    fn test_correct_answer_part_2() {
        assert_eq!(get_two_hundredth(), 1119);
    }
}
