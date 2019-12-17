// list of ints as input
// each phase, a new list is generated with the same length as the previous
// result of phase = summation of elements * pattern, only one's digit is preserved
use crate::utils::read::read_list;

struct PatternIter {
    subsequnce_length: usize,
    subsequence_index: usize,
    element_cycler: usize,
}

impl PatternIter {
    fn new(subsequnce_length: usize) -> PatternIter {
        PatternIter {
            subsequnce_length: subsequnce_length,
            subsequence_index: 0,
            element_cycler: 0,
        }
    }
}

impl Iterator for PatternIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let num = match self.element_cycler {
            0 => 0,
            1 => 1,
            2 => 0,
            3 => -1,
            _ => panic!("unexpected element cycler"),
        };
        self.subsequence_index += 1;

        if self.subsequence_index == self.subsequnce_length {
            self.element_cycler = (self.element_cycler + 1) % 4;
            self.subsequence_index = 0;
        }
        Some(num)
    }
}

fn perform_phase(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..nums.len() {
        let pattern = PatternIter::new(i + 1);
        let new_num = nums
            .iter()
            .cloned()
            .zip(pattern.skip(1))
            .map(|(a, b)| a * b)
            .fold(0, |acc, x| acc + x);
        result.push(new_num.abs() % 10);
    }
    result
}

fn read_input(s: &str) -> Vec<i32> {
    read_list(s, "")
}

fn perform_n_phases(n: usize, nums: Vec<i32>) -> Vec<i32> {
    let mut result = nums.clone();
    for _ in 0..n {
        result = perform_phase(result);
    }
    result
}

fn get_test_input() -> Vec<i32> {
    read_input(include_str!("day16_input.txt"))
}

pub fn perform_fft() -> String {
    let input = get_test_input();
    let result = perform_n_phases(100, input);
    result
        .into_iter()
        .take(8)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_generator() {
        {
            let iter = PatternIter::new(1);
            let result: Vec<_> = iter.skip(1).take(8).collect();
            assert_eq!(result, vec![1, 0, -1, 0, 1, 0, -1, 0]);
        }
        println!("-------------");
        {
            let iter = PatternIter::new(2);
            let result: Vec<_> = iter.skip(1).take(8).collect();
            assert_eq!(result, vec![0, 1, 1, 0, 0, -1, -1, 0]);
        }
        println!("-------------");
        {
            let iter = PatternIter::new(3);
            let result: Vec<_> = iter.skip(1).take(8).collect();
            assert_eq!(result, vec![0, 0, 1, 1, 1, 0, 0, 0]);
        }
        println!("-------------");
        {
            let iter = PatternIter::new(4);
            let result: Vec<_> = iter.skip(1).take(8).collect();
            assert_eq!(result, vec![0, 0, 0, 1, 1, 1, 1, 0]);
        }
        println!("-------------");
        {
            let iter = PatternIter::new(5);
            let result: Vec<_> = iter.skip(1).take(8).collect();
            assert_eq!(result, vec![0, 0, 0, 0, 1, 1, 1, 1]);
        }
        println!("-------------");
        {
            let iter = PatternIter::new(6);
            let result: Vec<_> = iter.skip(1).take(8).collect();
            assert_eq!(result, vec![0, 0, 0, 0, 0, 1, 1, 1]);
        }
        println!("-------------");
        {
            let iter = PatternIter::new(7);
            let result: Vec<_> = iter.skip(1).take(8).collect();
            assert_eq!(result, vec![0, 0, 0, 0, 0, 0, 1, 1]);
        }
        println!("-------------");
        {
            let iter = PatternIter::new(8);
            let result: Vec<_> = iter.skip(1).take(8).collect();
            assert_eq!(result, vec![0, 0, 0, 0, 0, 0, 0, 1]);
        }
    }

    #[test]
    fn test_perform_phase() {
        let list = read_input("12345678");
        let result = perform_phase(list);
        assert_eq!(result, read_input("48226158"));
    }

    #[test]
    fn test_perform_n_phases() {
        let list = read_input("12345678");
        let result = perform_n_phases(4, list);
        assert_eq!(result, read_input("01029498"));
    }
    #[test]
    fn test_large_known_cases() {
        {
            let result = perform_n_phases(100, read_input("80871224585914546619083218645595"))
                .into_iter()
                .take(8)
                .collect::<Vec<_>>();
            assert_eq!(result, read_input("24176176"));
        }
        {
            let result = perform_n_phases(100, read_input("19617804207202209144916044189917"))
                .into_iter()
                .take(8)
                .collect::<Vec<_>>();
            assert_eq!(result, read_input("73745418"));
        }
        {
            let result = perform_n_phases(100, read_input("69317163492948606335995924319873"))
                .into_iter()
                .take(8)
                .collect::<Vec<_>>();
            assert_eq!(result, read_input("52432133"));
        }
    }

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(perform_fft(), "45834272".to_string());
    }
}
