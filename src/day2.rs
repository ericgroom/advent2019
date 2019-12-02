use anyhow::Result;
use std::fs;

pub fn run_computation(input: Vec<i32>) -> Vec<i32> {
    let mut result = input.clone();
    for current_index in (0..result.len()).into_iter().step_by(4) {
        match result[current_index] {
            1 => {
                let first_index = result[current_index + 1] as usize;
                let second_index = result[current_index + 2] as usize;
                let result_index = result[current_index + 3] as usize;
                result[result_index] = result[first_index] + result[second_index];
            }
            2 => {
                let first_index = result[current_index + 1] as usize;
                let second_index = result[current_index + 2] as usize;
                let result_index = result[current_index + 3] as usize;
                result[result_index] = result[first_index] * result[second_index];
            }
            99 => return result,
            _ => {}
        }
    }
    return result;
}

fn read_input() -> Result<Vec<i32>> {
    let input = fs::read_to_string("./src/day2_input.txt")?;
    Ok(input
        .split(",")
        .filter_map(|word| word.parse::<i32>().ok())
        .collect())
}

pub fn restore_gravity_assist() -> Result<i32> {
    let mut input = read_input()?;
    input[1] = 12;
    input[2] = 2;
    let result = run_computation(input);
    Ok(result[0])
}

pub fn find_noun_and_verb() -> Result<(i32, i32)> {
    let input = read_input()?;
    let target = 19690720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = input.clone();
            memory[1] = noun;
            memory[2] = verb;
            if run_computation(memory)[0] == target {
                return Ok((noun, verb));
            }
        }
    }
    Ok((-1, -1))
}

pub fn noun_and_verb_result() -> Result<i32> {
    let (noun, verb) = find_noun_and_verb()?;
    Ok(100 * noun + verb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_cases() {
        assert_eq!(run_computation(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(run_computation(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            run_computation(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );

        assert_eq!(
            run_computation(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
        assert_eq!(
            run_computation(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_restore_gravity_assist_answer() -> Result<()> {
        assert_eq!(restore_gravity_assist()?, 5305097);
        Ok(())
    }

    #[test]
    fn test_noun_verb_answer() -> Result<()> {
        assert_eq!(noun_and_verb_result()?, 4925);
        Ok(())
    }
}
