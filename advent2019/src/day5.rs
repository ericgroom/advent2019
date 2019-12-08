extern crate anyhow;

use anyhow::Result;
use std::cell::Cell;

use crate::utils::read::read_list_from_file;
use intcode_computer::{Computer, IntCodeComputer};

pub fn run_diagnostic() -> Result<i32> {
    let input = read_list_from_file("./src/day5_input.txt", ",")?;
    let output_container = Cell::new(None);
    let output_callback = |num| {
        output_container.replace(Some(num));
    };
    let computer = IntCodeComputer::new(input, &output_callback);
    while computer.execute() {
        computer.provide_input(1);
    }
    Ok(output_container
        .take()
        .expect("Computer should have outputted a diagnostic value"))
}

pub fn run_test_diagnostic() -> Result<i32> {
    let input = read_list_from_file("./src/day5_input.txt", ",")?;
    let output_container = Cell::new(None);
    let output_callback = |num| {
        output_container.replace(Some(num));
    };
    let computer = IntCodeComputer::new(input, &output_callback);
    while computer.execute() {
        computer.provide_input(5);
    }
    Ok(output_container
        .take()
        .expect("Computer should have outputted a diagnostic value"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_execute_empty() {
        let computer = IntCodeComputer::new(vec![], &|_| {});
        let interrupted = computer.execute();
        assert_eq!(interrupted, false);
        assert_eq!(computer.terminate(), vec![]);
    }

    #[test]
    fn test_produces_same_output_as_old_computer() {
        let output = |_| panic!("output should never be called");
        {
            let computer = IntCodeComputer::new(vec![1, 0, 0, 0, 99], &output);
            assert_eq!(computer.execute(), false);
            let end_memory = computer.terminate();
            assert_eq!(end_memory, vec![2, 0, 0, 0, 99]);
        }
        {
            let computer = IntCodeComputer::new(vec![2, 3, 0, 3, 99], &output);
            assert_eq!(computer.execute(), false);
            let end_memory = computer.terminate();
            assert_eq!(end_memory, vec![2, 3, 0, 6, 99]);
        }
        {
            let computer = IntCodeComputer::new(vec![2, 4, 4, 5, 99, 0], &output);
            assert_eq!(computer.execute(), false);
            let end_memory = computer.terminate();
            assert_eq!(end_memory, vec![2, 4, 4, 5, 99, 9801]);
        }
        {
            let computer = IntCodeComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], &output);
            assert_eq!(computer.execute(), false);
            let end_memory = computer.terminate();
            assert_eq!(end_memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
        }
        {
            let computer =
                IntCodeComputer::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], &output);
            assert_eq!(computer.execute(), false);
            let end_memory = computer.terminate();
            assert_eq!(
                end_memory,
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
            );
        }
    }

    #[test]
    fn test_known_new_cases() {
        let output = |_| {};

        {
            let computer = IntCodeComputer::new(vec![1002, 4, 3, 4, 33], &output);
            assert_eq!(computer.execute(), false);
            let end_memory = computer.terminate();
            assert_eq!(end_memory, vec![1002, 4, 3, 4, 99]);
        }

        {
            let computer = IntCodeComputer::new(vec![1101, 100, -1, 4, 0], &output);
            assert_eq!(computer.execute(), false);
            let end_memory = computer.terminate();
            assert_eq!(end_memory, vec![1101, 100, -1, 4, 99]);
        }

        let output_called_count = Cell::new(0);
        let known_output = |num: i32| {
            assert_eq!(num, 2);
            output_called_count.replace(output_called_count.get() + 1);
        };
        let computer = IntCodeComputer::new(vec![3, 0, 4, 0, 99], &known_output);
        while computer.execute() {
            computer.provide_input(2);
        }

        assert_eq!(computer.terminate(), vec![2, 0, 4, 0, 99]);
        assert_eq!(output_called_count.take(), 1);
    }

    #[test]
    fn test_correct_answer_part_1() -> Result<()> {
        assert_eq!(run_diagnostic()?, 9938601);
        Ok(())
    }

    #[test]
    fn test_known_comparison_cases() {
        let equals_eight_true = {
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            let computer = IntCodeComputer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &output);
            while computer.execute() {
                computer.provide_input(8);
            }
            output_container.take()
        };
        assert_eq!(equals_eight_true, 1);

        let equals_eight_false = {
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            let computer = IntCodeComputer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &output);
            while computer.execute() {
                computer.provide_input(7);
            }
            output_container.take()
        };
        assert_eq!(equals_eight_false, 0);

        let equals_eight_true_immediate = {
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            let computer = IntCodeComputer::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], &output);
            while computer.execute() {
                computer.provide_input(8);
            }
            output_container.take()
        };
        assert_eq!(equals_eight_true_immediate, 1);

        let lt_eight_true_immediate = {
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            let computer = IntCodeComputer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], &output);
            while computer.execute() {
                computer.provide_input(7);
            }
            output_container.take()
        };
        assert_eq!(lt_eight_true_immediate, 1);
    }

    #[test]
    fn test_known_jump_cases() {
        let jump_zero_ptr = {
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            let computer = IntCodeComputer::new(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                &output,
            );
            while computer.execute() {
                computer.provide_input(0);
            }
            output_container.take()
        };
        assert_eq!(jump_zero_ptr, 0);

        let jump_gt_zero_ptr = {
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            let computer = IntCodeComputer::new(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                &output,
            );
            while computer.execute() {
                computer.provide_input(20);
            }
            output_container.take()
        };
        assert_eq!(jump_gt_zero_ptr, 1);

        let jump_zero_imm = {
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            let computer = IntCodeComputer::new(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                &output,
            );
            while computer.execute() {
                computer.provide_input(0);
            }
            output_container.take()
        };
        assert_eq!(jump_zero_imm, 0);

        let jump_gt_zero_imm = {
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            let computer = IntCodeComputer::new(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                &output,
            );
            while computer.execute() {
                computer.provide_input(20);
            }
            output_container.take()
        };
        assert_eq!(jump_gt_zero_imm, 1);
    }

    #[test]
    fn test_correct_answer_part_2() -> Result<()> {
        assert_eq!(run_test_diagnostic()?, 4283952);
        Ok(())
    }
}
