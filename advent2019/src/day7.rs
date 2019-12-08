extern crate anyhow;

use crate::utils::read::read_list_from_file;
use anyhow::Result;
use intcode_computer::pipe::Pipe;
use intcode_computer::{Computer, IntCodeComputer};
use std::cell::Cell;

fn get_amplifier_sequence_output(software: &Vec<i32>, phase_settings: &Vec<i32>) -> i32 {
    // first amp only: input signal 0
    // every amp: phase setting then input signal, outputs output signal
    let input_signal = Cell::new(0);
    for phase_setting in phase_settings.iter().cloned() {
        let memory = software.clone();
        let output_callback = |x| {
            input_signal.replace(x);
        };
        let computer = IntCodeComputer::new(memory, &output_callback);
        computer.provide_input(phase_setting);
        computer.provide_input(input_signal.get());
        while computer.execute() {
            println!("interupted")
        }
    }
    input_signal.get()
}

fn get_permutations(list: Vec<i32>) -> Vec<Vec<i32>> {
    if list.len() <= 1 {
        return vec![list];
    }
    let mut result = Vec::new();
    let head = list[0];
    let tail_permutations = get_permutations(list[1..].to_vec());
    for permutation in tail_permutations {
        for i in 0..list.len() {
            let mut temp_result = Vec::with_capacity(list.len());
            temp_result.extend(permutation[..i].iter());
            temp_result.push(head);
            temp_result.extend(permutation[i..].iter());
            result.push(temp_result);
        }
    }
    result
}

pub fn find_max_amplitude(software: Vec<i32>, num_amps: i32) -> i32 {
    let phase_settings = get_permutations((0..num_amps).collect());
    phase_settings
        .iter()
        .map(|phase_setting| get_amplifier_sequence_output(&software, phase_setting))
        .max()
        .unwrap()
}

pub fn find_highest_thruster_signal() -> Result<i32> {
    let input = read_list_from_file("./src/day7_input.txt", ",")?;
    Ok(find_max_amplitude(input, 5))
}

pub fn find_feedback_output(software: &Vec<i32>, phase_settings: Vec<i32>) -> i32 {
    let pipe = Pipe::new();
    let output = |x| pipe.send(x);
    let amps: Vec<IntCodeComputer> = phase_settings
        .iter()
        .map(|_| IntCodeComputer::new(software.clone(), &output))
        .collect();
    for (amp, phase) in amps.iter().zip(phase_settings.iter()) {
        amp.provide_input(*phase);
    }
    amps[0].provide_input(0);
    let mut halt_count = 0;
    while halt_count < amps.len() {
        for amp in &amps {
            if !pipe.is_empty() {
                let input = pipe.receive();
                amp.provide_input(input);
            }
            let has_halted = !amp.execute();
            if has_halted {
                halt_count += 1
            }
        }
    }
    let last_output = pipe.receive();
    last_output
}

pub fn find_max_feedback_output(software: Vec<i32>) -> i32 {
    let permutations = get_permutations((5..=9).collect());
    permutations
        .into_iter()
        .map(|permutation| find_feedback_output(&software, permutation))
        .max()
        .unwrap()
}

pub fn find_feedback_loop_max() -> Result<i32> {
    let input = read_list_from_file("./src/day7_input.txt", ",")?;
    Ok(find_max_feedback_output(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_cases() {
        {
            let memory = vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ];
            let output_signal = get_amplifier_sequence_output(&memory, &vec![4, 3, 2, 1, 0]);
            assert_eq!(output_signal, 43210);
        }
        {
            let memory = vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ];
            let output_signal = get_amplifier_sequence_output(&memory, &vec![0, 1, 2, 3, 4]);
            assert_eq!(output_signal, 54321);
        }
        {
            let memory = vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ];
            let output_signal = get_amplifier_sequence_output(&memory, &vec![1, 0, 4, 3, 2]);
            assert_eq!(output_signal, 65210);
        }
    }

    #[test]
    fn test_get_permutations() {
        let permutations = get_permutations(vec![1, 2, 3]);
        assert_eq!(permutations.len(), 6);
        assert_eq!(permutations.contains(&vec![1, 2, 3]), true);
        assert_eq!(permutations.contains(&vec![1, 3, 2]), true);
        assert_eq!(permutations.contains(&vec![2, 1, 3]), true);
        assert_eq!(permutations.contains(&vec![2, 3, 1]), true);
        assert_eq!(permutations.contains(&vec![3, 1, 2]), true);
        assert_eq!(permutations.contains(&vec![3, 2, 1]), true);

        assert_eq!(get_permutations(vec![1, 2, 3, 4, 5]).len(), 120);
    }

    #[test]
    fn test_correct_answer_part_1() -> Result<()> {
        assert_eq!(find_highest_thruster_signal()?, 199988);
        Ok(())
    }

    #[test]
    fn test_find_feedback_output() {
        let amplitude = find_feedback_output(
            &vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5,
            ],
            vec![9, 8, 7, 6, 5],
        );
        assert_eq!(amplitude, 139629729);
    }

    #[test]
    fn test_find_feedback_output_long() {
        let amplitude = find_feedback_output(
            &vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
            ],
            vec![9, 7, 8, 5, 6],
        );
        assert_eq!(amplitude, 18216);
    }

    #[test]
    fn test_correct_answer_part_2() -> Result<()> {
        assert_eq!(find_feedback_loop_max()?, 17519904);
        Ok(())
    }
}
