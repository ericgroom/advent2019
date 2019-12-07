extern crate anyhow;

use anyhow::Result;
use std::cell::Cell;

use crate::intcode_computer::{Computer, IntCodeComputer};
use crate::utils::read::read_list_from_file;

pub fn run_diagnostic() -> Result<i32> {
    let input = read_list_from_file("./src/day5_input.txt", ",")?;
    let input_instr_provider = || 1;
    let output_container = Cell::new(None);
    let output_callback = |num| {
        output_container.replace(Some(num));
    };
    let computer = IntCodeComputer::new(input, &input_instr_provider, &output_callback);
    computer.execute();
    Ok(output_container
        .take()
        .expect("Computer should have outputted a diagnostic value"))
}

pub fn run_test_diagnostic() -> Result<i32> {
    let input = read_list_from_file("./src/day5_input.txt", ",")?;
    let input_instr_provider = || 5;
    let output_container = Cell::new(None);
    let output_callback = |num| {
        output_container.replace(Some(num));
    };
    let computer = IntCodeComputer::new(input, &input_instr_provider, &output_callback);
    computer.execute();
    Ok(output_container
        .take()
        .expect("Computer should have outputted a diagnostic value"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_execute_empty() {
        let computer = IntCodeComputer::new(vec![], &|| 3, &|_| {});
        let output = computer.execute();
        assert_eq!(output, vec![]);
    }

    #[test]
    fn test_produces_same_output_as_old_computer() {
        let output = |_| {};
        let input: Box<dyn Fn() -> i32> = Box::new(|| panic!("input should never be called!"));

        assert_eq!(
            IntCodeComputer::new(vec![1, 0, 0, 0, 99], &input, &output).execute(),
            vec![2, 0, 0, 0, 99]
        );
        assert_eq!(
            IntCodeComputer::new(vec![2, 3, 0, 3, 99], &input, &output).execute(),
            vec![2, 3, 0, 6, 99]
        );
        assert_eq!(
            IntCodeComputer::new(vec![2, 4, 4, 5, 99, 0], &input, &output).execute(),
            vec![2, 4, 4, 5, 99, 9801]
        );

        assert_eq!(
            IntCodeComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], &input, &output).execute(),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
        assert_eq!(
            IntCodeComputer::new(
                vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                &input,
                &output
            )
            .execute(),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_known_new_cases() {
        let output = |_| {};
        let input: Box<dyn Fn() -> i32> = Box::new(|| panic!("input should never be called!"));

        assert_eq!(
            IntCodeComputer::new(vec![1002, 4, 3, 4, 33], &input, &output).execute(),
            vec![1002, 4, 3, 4, 99]
        );

        assert_eq!(
            IntCodeComputer::new(vec![1101, 100, -1, 4, 0], &input, &output).execute(),
            vec![1101, 100, -1, 4, 99]
        );

        let output_called_count = Cell::new(0);
        let known_input = || 2;
        let known_output = |num: i32| {
            assert_eq!(num, 2);
            output_called_count.replace(output_called_count.get() + 1);
        };
        assert_eq!(
            IntCodeComputer::new(vec![3, 0, 4, 0, 99], &known_input, &known_output).execute(),
            vec![2, 0, 4, 0, 99]
        );
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
            let input = || 8;
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            IntCodeComputer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &input, &output)
                .execute();
            output_container.take()
        };
        assert_eq!(equals_eight_true, 1);

        let equals_eight_false = {
            let input = || 7;
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            IntCodeComputer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &input, &output)
                .execute();
            output_container.take()
        };
        assert_eq!(equals_eight_false, 0);

        let equals_eight_true_immediate = {
            let input = || 8;
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            IntCodeComputer::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], &input, &output).execute();
            output_container.take()
        };
        assert_eq!(equals_eight_true_immediate, 1);

        let lt_eight_true_immediate = {
            let input = || 7;
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            IntCodeComputer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], &input, &output).execute();
            output_container.take()
        };
        assert_eq!(lt_eight_true_immediate, 1);
    }

    #[test]
    fn test_known_jump_cases() {
        let jump_zero_ptr = {
            let input = || 0;
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            IntCodeComputer::new(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                &input,
                &output,
            )
            .execute();
            output_container.take()
        };
        assert_eq!(jump_zero_ptr, 0);

        let jump_gt_zero_ptr = {
            let input = || 20;
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            IntCodeComputer::new(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                &input,
                &output,
            )
            .execute();
            output_container.take()
        };
        assert_eq!(jump_gt_zero_ptr, 1);

        let jump_zero_imm = {
            let input = || 0;
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            IntCodeComputer::new(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                &input,
                &output,
            )
            .execute();
            output_container.take()
        };
        assert_eq!(jump_zero_imm, 0);

        let jump_gt_zero_imm = {
            let input = || 20;
            let output_container = Cell::new(-1);
            let output = |num| {
                output_container.replace(num);
            };
            IntCodeComputer::new(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                &input,
                &output,
            )
            .execute();
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
