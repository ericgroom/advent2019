fn run_computation(input: Vec<i32>) -> Vec<i32> {
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
}
