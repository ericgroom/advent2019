use crate::utils::read::read_list;
use intcode_computer::prelude::*;

fn get_test_input() -> IntcodeMemoryType {
    read_list(include_str!("./day5_input.txt"), ",")
}

pub fn run_diagnostic() -> IntcodeMemoryCellType {
    let input = get_test_input();
    let mut computer = IntCodeComputer::new(input);
    let mut output = -1;
    execute! { computer,
        input { computer.provide_input(1) },
        output { output = computer.take_output() }
    }
    output
}

pub fn run_test_diagnostic() -> IntcodeMemoryCellType {
    let input = get_test_input();
    let mut computer = IntCodeComputer::new(input);
    execute! { computer,
        input { computer.provide_input(5) },
        output { return computer.take_output() }
    }
    panic!("no diagnostic code provided");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_execute_empty() {
        let mut computer = IntCodeComputer::new(vec![]);
        let interrupted = computer.execute();
        assert_eq!(interrupted, Interrupt::Halt);
        assert_eq!(computer.terminate(), vec![]);
    }

    #[test]
    fn test_produces_same_output_as_old_computer() {
        {
            let mut computer = IntCodeComputer::new(vec![1, 0, 0, 0, 99]);
            execute!(computer);
            let end_memory = computer.terminate();
            assert_eq!(end_memory, vec![2, 0, 0, 0, 99]);
        }
        {
            let mut computer = IntCodeComputer::new(vec![2, 3, 0, 3, 99]);
            execute!(computer);
            let end_memory = computer.terminate();
            assert_eq!(end_memory, vec![2, 3, 0, 6, 99]);
        }
        {
            let mut computer = IntCodeComputer::new(vec![2, 4, 4, 5, 99, 0]);
            execute!(computer);
            let end_memory = computer.terminate();
            assert_eq!(end_memory, vec![2, 4, 4, 5, 99, 9801]);
        }
        {
            let mut computer = IntCodeComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
            execute!(computer);
            let end_memory = computer.terminate();
            assert_eq!(end_memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
        }
        {
            let mut computer = IntCodeComputer::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
            execute!(computer);
            let end_memory = computer.terminate();
            assert_eq!(
                end_memory,
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
            );
        }
    }

    #[test]
    fn test_known_new_cases() {
        {
            let mut computer = IntCodeComputer::new(vec![1002, 4, 3, 4, 33]);
            execute!(computer);
            let end_memory = computer.terminate();
            assert_eq!(end_memory, vec![1002, 4, 3, 4, 99]);
        }

        {
            let mut computer = IntCodeComputer::new(vec![1101, 100, -1, 4, 0]);
            execute!(computer);
            let end_memory = computer.terminate();
            assert_eq!(end_memory, vec![1101, 100, -1, 4, 99]);
        }

        let mut output_called_count = 0;
        let mut computer = IntCodeComputer::new(vec![3, 0, 4, 0, 99]);
        execute! { computer,
            output {
                output_called_count += 1;
                assert_eq!(computer.take_output(), 2);
            },
            input { computer.provide_input(2) }
        }

        assert_eq!(computer.terminate(), vec![2, 0, 4, 0, 99]);
        assert_eq!(output_called_count, 1);
    }

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(run_diagnostic(), 9938601);
    }

    #[test]
    fn test_known_comparison_cases() {
        let equals_eight_true: i64 = {
            let mut output = -1;
            let mut computer = IntCodeComputer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
            execute! { computer,
                    output { output = computer.take_output() },
                    input { computer.provide_input(8) }
            }
            output
        };
        assert_eq!(equals_eight_true, 1);

        let equals_eight_false = {
            let mut output = -1;
            let mut computer = IntCodeComputer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
            execute! { computer,
                    output { output = computer.take_output() },
                    input { computer.provide_input(7) }
            }
            output
        };
        assert_eq!(equals_eight_false, 0);

        let equals_eight_true_immediate = {
            let mut output = -1;
            let mut computer = IntCodeComputer::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
            execute! { computer,
                    output { output = computer.take_output() },
                    input { computer.provide_input(8) }
            }
            output
        };
        assert_eq!(equals_eight_true_immediate, 1);

        let lt_eight_true_immediate = {
            let mut output = -1;
            let mut computer = IntCodeComputer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
            execute! { computer,
                    output { output = computer.take_output() },
                    input { computer.provide_input(7) }
            }
            output
        };
        assert_eq!(lt_eight_true_immediate, 1);
    }

    #[test]
    fn test_known_jump_cases() {
        let jump_zero_ptr = {
            let mut output = -1;
            let mut computer = IntCodeComputer::new(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
            ]);
            execute! { computer,
                    output { output = computer.take_output() },
                    input { computer.provide_input(0) }
            }
            output
        };
        assert_eq!(jump_zero_ptr, 0);

        let jump_gt_zero_ptr = {
            let mut output = -1;
            let mut computer = IntCodeComputer::new(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
            ]);
            execute! { computer,
                    output { output = computer.take_output() },
                    input { computer.provide_input(20) }
            }
            output
        };
        assert_eq!(jump_gt_zero_ptr, 1);

        let jump_zero_imm = {
            let mut output = -1;
            let mut computer =
                IntCodeComputer::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
            execute! { computer,
                    output { output = computer.take_output() },
                    input { computer.provide_input(0) }
            }
            output
        };
        assert_eq!(jump_zero_imm, 0);

        let jump_gt_zero_imm = {
            let mut output = -1;
            let mut computer =
                IntCodeComputer::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
            execute! { computer,
                    output { output = computer.take_output() },
                    input { computer.provide_input(20) }
            }
            output
        };
        assert_eq!(jump_gt_zero_imm, 1);
    }

    #[test]
    fn test_correct_answer_part_2() {
        assert_eq!(run_test_diagnostic(), 4283952);
    }
}
