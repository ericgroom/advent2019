use crate::utils::read::read_list;
use intcode_computer::{Computer, IntCodeComputer, IntcodeMemoryCellType, IntcodeMemoryType};
use std::cell::{Cell, RefCell};

fn get_test_input() -> IntcodeMemoryType {
    read_list(include_str!("./day9_input.txt"), ",")
}

pub fn run_boost_diagnostic() -> IntcodeMemoryCellType {
    let mut memory = get_test_input();
    memory.resize(memory.len() + 256, 0);
    let output_container = RefCell::new(Vec::new());
    let output_handle = |i| output_container.borrow_mut().push(i);
    let computer = IntCodeComputer::new(memory, &output_handle);
    computer.provide_input(1);
    while computer.execute() {}
    let outputs = output_container.into_inner();
    match &outputs[..] {
        [keycode] => *keycode,
        list => panic!("diagnostic failed, outputs: {:?}", list),
    }
}

pub fn get_distress_signal_coords() -> IntcodeMemoryCellType {
    let mut memory = get_test_input();
    memory.resize(memory.len() + 256, 0);
    let output_container = Cell::new(0);
    let output_handle = |i| output_container.set(i);
    let computer = IntCodeComputer::new(memory, &output_handle);
    computer.provide_input(2);
    while computer.execute() {}
    output_container.into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[test]
    fn test_relative_mode() {
        let mut program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let program_copy = program.clone();

        program.resize(program.len() + 256, 0);
        let output_container = RefCell::new(Vec::new());
        let output_handle = |i| output_container.borrow_mut().push(i);
        let computer = IntCodeComputer::new(program, &output_handle);
        while computer.execute() {}
        assert_eq!(program_copy, output_container.into_inner());
    }

    #[test]
    fn test_large_output() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let output_container = Cell::new(0);
        let output_handle = |i| {
            output_container.replace(i);
        };
        let computer = IntCodeComputer::new(program, &output_handle);
        while computer.execute() {}
        let mut output_digit = output_container.into_inner();
        let mut digit_count = 0;
        while output_digit != 0 {
            digit_count += 1;
            output_digit /= 10;
        }
        assert_eq!(digit_count, 16);
    }

    #[test]
    fn test_large_output_in_middle() {
        let program = vec![104, 1125899906842624, 99];
        let output_container = Cell::new(0);
        let output_handle = |i| {
            output_container.replace(i);
        };
        let computer = IntCodeComputer::new(program, &output_handle);
        while computer.execute() {}
        assert_eq!(output_container.into_inner(), 1125899906842624);
    }

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(run_boost_diagnostic(), 2671328082);
    }

    #[test]
    fn test_correct_answer_part_2() {
        assert_eq!(get_distress_signal_coords(), 59095);
    }
}
