extern crate anyhow;

use anyhow::*;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Movement {
    direction: Direction,
    magnitude: i32,
}

impl FromStr for Movement {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(anyhow!("Empty string"));
        }
        let trimmed = s.trim();
        let direction = match trimmed.chars().next() {
            None => return Err(anyhow!("Empty string")),
            Some(char_) => match char_ {
                'R' => Direction::Right,
                'L' => Direction::Left,
                'U' => Direction::Up,
                'D' => Direction::Down,
                _ => return Err(anyhow!("Direction {} not found", char_)),
            },
        };
        let number_segment = trimmed.get(1..);
        let magnitude = match number_segment {
            None => return Err(anyhow!("no number segment")),
            Some(num_str) => num_str.parse::<i32>(),
        }?;
        Ok(Movement {
            direction: direction,
            magnitude: magnitude,
        })
    }
}

impl Movement {
    fn to_xy(&self) -> (i32, i32) {
        match self.direction {
            Direction::Left => (-self.magnitude, 0),
            Direction::Right => (self.magnitude, 0),
            Direction::Up => (0, self.magnitude),
            Direction::Down => (0, -self.magnitude),
        }
    }

    fn normalize(&self) -> Vec<Movement> {
        std::iter::repeat(Movement {
            direction: self.direction,
            magnitude: 1,
        })
        .take(self.magnitude as usize)
        .collect()
    }
}

fn find_nearest_wire_intersection(wire1: Vec<Movement>, wire2: Vec<Movement>) -> i32 {
    let wire1_movements: Vec<Movement> = wire1.iter().flat_map(Movement::normalize).collect();
    let wire2_movements: Vec<Movement> = wire2.iter().flat_map(Movement::normalize).collect();

    let wire1_set = visited_nodes(wire1_movements);
    let wire2_set = visited_nodes(wire2_movements);
    let intersection: Vec<(i32, i32)> = wire1_set.intersection(&wire2_set).cloned().collect();
    intersection
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .expect("at least one intersection")
}

fn visited_nodes(movements: Vec<Movement>) -> HashSet<(i32, i32)> {
    let mut position = (0, 0);
    let mut result_set = HashSet::new();
    for movement in movements {
        let (dx, dy) = movement.to_xy();
        position.0 += dx;
        position.1 += dy;
        result_set.insert(position);
    }
    result_set
}

fn read_input_file() -> Result<(Vec<Movement>, Vec<Movement>)> {
    let input = std::fs::read_to_string("./src/day3_input.txt")?;
    Ok(read_input(&input))
}

fn read_input(input: &str) -> (Vec<Movement>, Vec<Movement>) {
    let mut input_iter = input.split('\n');
    let wire1_str = input_iter.next().expect("no input for wire1");
    let wire2_str = input_iter.next().expect("no input for wire2");
    let wire1_movements: Vec<Movement> = wire1_str
        .split(',')
        .map(Movement::from_str)
        .filter_map(std::result::Result::ok)
        .collect();
    let wire2_movements: Vec<Movement> = wire2_str
        .split(',')
        .map(Movement::from_str)
        .filter_map(std::result::Result::ok)
        .collect();
    (wire1_movements, wire2_movements)
}

pub fn closest_intersection() -> Result<i32> {
    let (wire1, wire2) = read_input_file()?;
    Ok(find_nearest_wire_intersection(wire1, wire2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn test_movement_from_str() -> anyhow::Result<()> {
        assert_eq!(
            Movement::from_str("R2")?,
            Movement {
                direction: Direction::Right,
                magnitude: 2
            }
        );
        assert_eq!(
            Movement::from_str("R99")?,
            Movement {
                direction: Direction::Right,
                magnitude: 99
            }
        );
        assert_eq!(
            Movement::from_str("L3")?,
            Movement {
                direction: Direction::Left,
                magnitude: 3
            }
        );
        assert_eq!(
            Movement::from_str("U5")?,
            Movement {
                direction: Direction::Up,
                magnitude: 5
            }
        );
        assert_eq!(
            Movement::from_str("D7")?,
            Movement {
                direction: Direction::Down,
                magnitude: 7
            }
        );
        Ok(())
    }

    #[test]
    fn test_movement_normalize() {
        let movement = Movement {
            direction: Direction::Up,
            magnitude: 3,
        };
        let expected_movement_factory = || Movement {
            direction: Direction::Up,
            magnitude: 1,
        };
        assert_eq!(
            movement.normalize(),
            vec![
                expected_movement_factory(),
                expected_movement_factory(),
                expected_movement_factory()
            ]
        )
    }

    #[test]
    fn test_find_nearest_intersection() {
        let (wire1, wire2) =
            read_input("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
        let distance = find_nearest_wire_intersection(wire1, wire2);
        assert_eq!(distance, 159);
    }
    #[test]
    fn test_find_nearest_intersection_simple() {
        let (wire1, wire2) = read_input("R2,U3\nU2,R2");
        let distance = find_nearest_wire_intersection(wire1, wire2);
        assert_eq!(distance, 4);
    }

    #[test]
    fn test_correct_answer() -> Result<()> {
        assert_eq!(closest_intersection()?, 1431);
        Ok(())
    }
}
