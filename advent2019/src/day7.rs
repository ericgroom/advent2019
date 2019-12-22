use crate::utils::read::read_list;
use intcode_computer::pipe::Pipe;
use intcode_computer::prelude::*;

fn get_amplifier_sequence_output(
    software: &IntcodeMemoryType,
    phase_settings: &Vec<i32>,
) -> IntcodeMemoryCellType {
    // first amp only: input signal 0
    // every amp: phase setting then input signal, outputs output signal
    let mut input_signal = 0;
    for phase_setting in phase_settings.iter().cloned() {
        let memory = software.clone();
        let mut computer = IntCodeComputer::new(memory);
        computer.provide_input(phase_setting.into());
        computer.provide_input(input_signal);
        execute! { computer,
            output { input_signal = computer.take_output() }
        }
    }
    input_signal
}

fn get_permutations<T: Copy>(list: Vec<T>) -> Vec<Vec<T>> {
    if list.len() <= 1 {
        return vec![list];
    }
    let mut result = Vec::new();
    let head = list[0];
    let tail_permutations = get_permutations(list[1..].to_vec());
    for permutation in tail_permutations {
        for i in 0..list.len() {
            let mut temp_result: Vec<T> = Vec::with_capacity(list.len());
            temp_result.extend_from_slice(&permutation[..i]);
            temp_result.push(head);
            temp_result.extend_from_slice(&permutation[i..]);
            result.push(temp_result);
        }
    }
    result
}

pub fn find_max_amplitude(software: IntcodeMemoryType, num_amps: i32) -> IntcodeMemoryCellType {
    let phase_settings = get_permutations((0..num_amps).collect());
    phase_settings
        .iter()
        .map(|phase_setting| get_amplifier_sequence_output(&software, phase_setting))
        .max()
        .unwrap()
}

fn get_test_input() -> IntcodeMemoryType {
    read_list(include_str!("./day7_input.txt"), ",")
}

pub fn find_highest_thruster_signal() -> IntcodeMemoryCellType {
    let input = get_test_input();
    find_max_amplitude(input, 5)
}

pub fn find_feedback_output(
    software: &IntcodeMemoryType,
    phase_settings: Vec<i32>,
) -> IntcodeMemoryCellType {
    let pipe = Pipe::new();
    let mut amps: Vec<IntCodeComputer> = phase_settings
        .iter()
        .map(|_| IntCodeComputer::new(software.clone()))
        .collect();
    for (amp, phase) in amps.iter_mut().zip(phase_settings.iter()) {
        amp.provide_input((*phase).into());
    }
    amps[0].provide_input(0);
    let mut halt_count = 0;
    while halt_count < amps.len() {
        for amp in amps.iter_mut() {
            if !pipe.is_empty() {
                let input = pipe.receive();
                amp.provide_input(input.into());
            }
            loop {
                let interrupt = amp.execute();
                match interrupt {
                    Interrupt::Output => pipe.send(amp.take_output()),
                    Interrupt::Halt => {
                        halt_count += 1;
                        break;
                    }
                    Interrupt::Input => break,
                };
            }
        }
    }
    let last_output = pipe.receive();
    last_output
}

pub fn find_max_feedback_output(software: IntcodeMemoryType) -> IntcodeMemoryCellType {
    let permutations = get_permutations((5..=9).collect());
    permutations
        .into_iter()
        .map(|permutation| find_feedback_output(&software, permutation))
        .max()
        .unwrap()
}

pub fn find_feedback_loop_max() -> IntcodeMemoryCellType {
    let input = get_test_input();
    find_max_feedback_output(input)
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
    fn test_correct_answer_part_1() {
        assert_eq!(find_highest_thruster_signal(), 199988);
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
    fn test_correct_answer_part_2() {
        assert_eq!(find_feedback_loop_max(), 17519904);
    }
}
