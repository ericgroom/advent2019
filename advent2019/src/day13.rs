use crate::utils::geometry::{render_image, Vec2D};
use crate::utils::read::read_list;
use intcode_computer::{Computer, IntCodeComputer, IntcodeMemoryCellType, IntcodeMemoryType};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Entity {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<i64> for Entity {
    fn from(tile_id: i64) -> Self {
        match tile_id {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::HorizontalPaddle,
            4 => Self::Ball,
            x => panic!("Tile id: {} does not exist", tile_id),
        }
    }
}

fn get_test_input() -> IntcodeMemoryType {
    read_list(include_str!("day13_input.txt"), ",")
}

fn run_game(game: IntcodeMemoryType) -> IntcodeMemoryType {
    let output_container = RefCell::new(Vec::new());
    let output_handle = |i| output_container.borrow_mut().push(i);
    let computer = IntCodeComputer::new(game, &output_handle);
    let has_halted = !computer.execute();
    assert!(has_halted);
    output_container.into_inner()
}

pub fn count_blocks() -> usize {
    let mut input = get_test_input();
    input.resize(3000, 0);
    let computer_output = run_game(input);
    assert_eq!(computer_output.len() % 3, 0);
    let mut grid = HashMap::new();
    for tile in computer_output[..].chunks(3) {
        let (x, y, tile_id) = (tile[0], tile[1], tile[2]);
        let point = Vec2D::new(x as i32, y as i32);
        let tile = Entity::from(tile_id);
        grid.insert(point, tile);
    }
    grid.values()
        .cloned()
        .filter(|tile| *tile == Entity::Block)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(count_blocks(), 207);
    }
}
