use crate::utils::geometry::render_image;
use crate::utils::geometry::{CardinalDirection, Vec2D};
use crate::utils::read::read_list;
use intcode_computer::{Computer, IntCodeComputer, IntcodeMemoryCellType, IntcodeMemoryType};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    White,
}

impl Into<IntcodeMemoryCellType> for Color {
    fn into(self) -> IntcodeMemoryCellType {
        match self {
            Self::Black => 0,
            Self::White => 1,
        }
    }
}

impl From<IntcodeMemoryCellType> for Color {
    fn from(memory: IntcodeMemoryCellType) -> Self {
        match memory {
            0 => Self::Black,
            1 => Self::White,
            x => panic!("Unexpected color: {}", x),
        }
    }
}

struct ShipHull {
    pub position_color_map: HashMap<Vec2D, Color>,
}

impl ShipHull {
    fn paint(&mut self, point: Vec2D, color: Color) {
        self.position_color_map.insert(point, color);
    }

    fn get_color(&mut self, point: &Vec2D) -> &Color {
        let current_value = { self.position_color_map.get(point) };
        match current_value {
            Some(color) => color,
            None => self.position_color_map.get(point).unwrap_or(&Color::Black),
        }
    }

    fn new() -> ShipHull {
        ShipHull {
            position_color_map: HashMap::new(),
        }
    }
}

enum RotationDirection {
    Right,
    Left,
}

struct EmergencyPaintingRobot {
    facing: CardinalDirection,
    location: Vec2D,
}

impl EmergencyPaintingRobot {
    fn new() -> EmergencyPaintingRobot {
        EmergencyPaintingRobot {
            facing: CardinalDirection::North,
            location: Vec2D::new(0, 0),
        }
    }
    fn advance(&mut self) -> &Vec2D {
        let new_location = match self.facing {
            CardinalDirection::North => Vec2D::new(self.location.x, self.location.y - 1),
            CardinalDirection::East => Vec2D::new(self.location.x + 1, self.location.y),
            CardinalDirection::South => Vec2D::new(self.location.x, self.location.y + 1),
            CardinalDirection::West => Vec2D::new(self.location.x - 1, self.location.y),
        };
        self.location = new_location;
        &self.location
    }

    fn rotate(&mut self, direction: RotationDirection) {
        let new_direction = match self.facing {
            CardinalDirection::North => match direction {
                RotationDirection::Left => CardinalDirection::West,
                RotationDirection::Right => CardinalDirection::East,
            },
            CardinalDirection::East => match direction {
                RotationDirection::Left => CardinalDirection::North,
                RotationDirection::Right => CardinalDirection::South,
            },
            CardinalDirection::South => match direction {
                RotationDirection::Left => CardinalDirection::East,
                RotationDirection::Right => CardinalDirection::West,
            },
            CardinalDirection::West => match direction {
                RotationDirection::Left => CardinalDirection::South,
                RotationDirection::Right => CardinalDirection::North,
            },
        };
        self.facing = new_direction;
    }
}

fn take_the_robot_for_a_walk(software: IntcodeMemoryType, hull: ShipHull) -> ShipHull {
    let output_buffer = RefCell::new(VecDeque::new());
    let output_handler = |i| output_buffer.borrow_mut().push_back(i);
    let computer = IntCodeComputer::new(software, &output_handler);
    let mut robot = EmergencyPaintingRobot::new();
    let mut hull = hull;

    let current_color = hull.get_color(&robot.location);
    computer.provide_input((*current_color).into());
    while computer.execute() {
        let mut output_buffer_m = output_buffer.borrow_mut();
        assert_eq!(output_buffer_m.len(), 2);
        let color_to_paint: Color = output_buffer_m.pop_front().unwrap().into();
        let direction_to_turn: RotationDirection = match output_buffer_m.pop_front().unwrap() {
            0 => RotationDirection::Left,
            1 => RotationDirection::Right,
            _ => panic!("invalid direction to turn"),
        };

        hull.paint(robot.location, color_to_paint);
        robot.rotate(direction_to_turn);
        robot.advance();

        // for next iter
        let current_color = hull.get_color(&robot.location);
        computer.provide_input((*current_color).into());
    }
    hull
}

fn get_test_input() -> IntcodeMemoryType {
    read_list(include_str!("./day11_input.txt"), ",")
}

pub fn get_coverage() -> usize {
    let mut software = get_test_input();
    software.resize(2000, 0);
    let hull = take_the_robot_for_a_walk(software, ShipHull::new());
    hull.position_color_map.len()
}

pub fn get_registration_identifier() -> String {
    let mut hull = ShipHull::new();
    let mut software = get_test_input();
    software.resize(2000, 0);
    hull.paint(Vec2D::new(0, 0), Color::White);
    let painted_hull = take_the_robot_for_a_walk(software, hull);

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
    for point in painted_hull.position_color_map.keys() {
        if point.x < min_x {
            min_x = point.x
        }
        if point.x > max_x {
            max_x = point.x
        }
        if point.y < min_y {
            min_y = point.y
        }
        if point.y > max_y {
            max_y = point.y
        }
    }
    let y_shift = if min_y < 0 { -min_y } else { 0 };
    let x_shift = if min_x < 0 { -min_x } else { 0 };
    let width = max_x - min_x;
    let mut result: Vec<i32> = Vec::new();
    result.resize(((max_x + x_shift + 1) * (max_y + y_shift + 1)) as usize, 0);
    for (point, color) in painted_hull.position_color_map {
        let index = (width * (point.y + y_shift) + (point.x + x_shift)) as usize;
        result[index] = match color {
            Color::Black => 0,
            Color::White => 1,
        }
    }
    render_image(result, width as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walking_the_robot() {
        let test_program = vec![1, 0, 0, 0, 3, 0, 104, 1, 104, 1, 1006, 0, 4, 99];
        let painted_hull = take_the_robot_for_a_walk(test_program, ShipHull::new());
        assert_eq!(painted_hull.position_color_map.len(), 4);
    }

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(get_coverage(), 1681);
    }

    #[test]
    fn test_correct_answer_part_2() {
        assert_eq!(
            get_registration_identifier(),
            "█    ██  ██    ██  ██   ██ ██ ██  ██ ██ ██
█ ████ ██ ████ █ ██ █ ██ █ █ ██ ██ █ █ ███
█   ██ ██████ ██ ████ ██ █  ███ ████  ████
█ ████ █  ██ ███ ████   ██ █ ██ █  █ █ ███
█ ████ ██ █ ████ ██ █ █ ██ █ ██ ██ █ █ ███
█    ██   █    ██  ██ ██ █ ██ ██   █ ██ ██
██████
"
        )
    }
}
