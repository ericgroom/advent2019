use crate::utils::geometry::{convert_map_to_grid, render_image};
use crate::utils::geometry::{CardinalDirection, Vec2D};
use crate::utils::read::read_list;
use intcode_computer::prelude::*;
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
        let new_direction = match direction {
            RotationDirection::Left => self.facing.counter_clockwise(),
            RotationDirection::Right => self.facing.clockwise(),
        };
        self.facing = new_direction;
    }
}

fn take_the_robot_for_a_walk(software: IntcodeMemoryType, hull: ShipHull) -> ShipHull {
    let mut output_buffer: VecDeque<IntcodeMemoryCellType> = VecDeque::new();
    let mut computer = IntCodeComputer::new(software);
    let mut robot = EmergencyPaintingRobot::new();
    let mut hull = hull;

    let current_color = hull.get_color(&robot.location);
    computer.provide_input((*current_color).into());
    execute! { computer,
        output {
            output_buffer.push_back(computer.take_output());
        },
        input {
            assert_eq!(output_buffer.len(), 2);
            let color_to_paint: Color = output_buffer.pop_front().unwrap().into();
            let direction_to_turn: RotationDirection = match output_buffer.pop_front().unwrap()
            {
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
    };
    hull
}

fn get_test_input() -> IntcodeMemoryType {
    read_list(include_str!("./day11_input.txt"), ",")
}

pub fn get_coverage() -> usize {
    let software = get_test_input();
    let hull = take_the_robot_for_a_walk(software, ShipHull::new());
    hull.position_color_map.len()
}

pub fn get_registration_identifier() -> String {
    let mut hull = ShipHull::new();
    let software = get_test_input();
    hull.paint(Vec2D::new(0, 0), Color::White);
    let painted_hull = take_the_robot_for_a_walk(software, hull);
    let convert_color = |color| Into::<i64>::into(color) as i32;
    let (width, grid) =
        convert_map_to_grid(&painted_hull.position_color_map, 0, Box::new(convert_color));
    render_image(grid, width, Box::new(render_pixel))
}

fn render_pixel(value: &i32) -> char {
    match value {
        0 => '█',
        1 => ' ',
        _ => panic!("pixel other than 0, 1, 2"),
    }
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
