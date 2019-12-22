use crate::utils::read::read_list;
use intcode_computer::prelude::*;

fn get_test_input() -> IntcodeMemoryType {
    read_list(include_str!("./day9_input.txt"), ",")
}

pub fn run_boost_diagnostic() -> IntcodeMemoryCellType {
    let memory = get_test_input();
    let mut outputs = Vec::new();
    let computer = IntCodeComputer::new(memory);
    computer.provide_input(1);
    execute! {computer,
        output { outputs.push(computer.take_output()) }
    }
    match &outputs[..] {
        [keycode] => *keycode,
        list => panic!("diagnostic failed, outputs: {:?}", list),
    }
}

pub fn get_distress_signal_coords() -> IntcodeMemoryCellType {
    let memory = get_test_input();
    let mut output = 0;
    let computer = IntCodeComputer::new(memory);
    computer.provide_input(2);
    execute! { computer,
        output { output = computer.take_output() }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_mode() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let program_copy = program.clone();

        let mut outputs = Vec::new();
        let computer = IntCodeComputer::new(program);
        execute! { computer,
            output { outputs.push(computer.take_output()) }
        }
        assert_eq!(program_copy, outputs);
    }

    #[test]
    fn test_large_output() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut output = 0;
        let computer = IntCodeComputer::new(program);
        execute! { computer,
            output { output = computer.take_output() }
        }
        let mut output_digit = output;
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
        let mut output = 0;
        let computer = IntCodeComputer::new(program);
        execute! { computer,
            output { output = computer.take_output() }
        }
        assert_eq!(output, 1125899906842624);
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
