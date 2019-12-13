use crate::utils::geometry::{convert_map_to_grid, render_image, Vec2D};
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
            _ => panic!("Tile id: {} does not exist", tile_id),
        }
    }
}

impl Into<i64> for Entity {
    fn into(self) -> i64 {
        match self {
            Self::Empty => 0,
            Self::Wall => 1,
            Self::Block => 2,
            Self::HorizontalPaddle => 3,
            Self::Ball => 4,
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
    let (_, map) = output_to_map(&computer_output);
    map.values()
        .cloned()
        .filter(|tile| *tile == Entity::Block)
        .count()
}

fn output_to_map(output: &IntcodeMemoryType) -> (i64, HashMap<Vec2D, Entity>) {
    assert_eq!(output.len() % 3, 0);
    let mut score = 0;
    let mut grid = HashMap::new();
    for tile in output[..].chunks(3) {
        let (x, y, tile_id) = (tile[0], tile[1], tile[2]);
        if x == -1 && y == 0 {
            score += tile_id;
            continue;
        }
        let point = Vec2D::new(x as i32, y as i32);
        let tile = Entity::from(tile_id);
        grid.insert(point, tile);
    }
    (score, grid)
}

pub fn play_game() {
    let mut game = get_test_input();
    game.resize(3000, 0);
    game[0] = 2;
    let output_container = RefCell::new(Vec::new());
    let output_handle = |i| output_container.borrow_mut().push(i);
    let computer = IntCodeComputer::new(game, &output_handle);
    while computer.execute() {
        computer.provide_input(0);
        let output = output_container.borrow_mut();
        if !output.is_empty() {
            let (score, map) = output_to_map(&output);
            let convert_entity = |entity| Into::<i64>::into(entity) as i32;
            let (width, grid) = convert_map_to_grid(map, 0, Box::new(convert_entity));
            let frame = render_image(grid, width, Box::new(render_pixel));
            println!("score: {}", score);
            println!("{}", frame);
        }
    }
}

fn render_pixel(value: &i32) -> char {
    match value {
        0 => ' ',
        1 => '|',
        2 => '█',
        3 => '_',
        4 => '•',
        _ => panic!("Cannot render {} as pixel", value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(count_blocks(), 207);
    }
}
