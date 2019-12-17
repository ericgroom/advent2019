// list of ints as input
// each phase, a new list is generated with the same length as the previous
// result of phase = summation of elements * pattern, only one's digit is preserved
use crate::utils::read::read_list;
use rayon::prelude::*;

struct PatternIter {
    subsequnce_length: usize,
    index: usize,
}

impl PatternIter {
    fn new(subsequnce_length: usize) -> PatternIter {
        PatternIter {
            subsequnce_length: subsequnce_length,
            index: 0,
        }
    }
}

impl Iterator for PatternIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let num = match self.index / self.subsequnce_length % 4 {
            0 => 0,
            1 => 1,
            2 => 0,
            3 => -1,
            _ => panic!("unexpected element cycler"),
        };
        self.index += 1;
        Some(num)
    }
}

fn perform_phase(nums: Vec<i32>, skip: usize) -> Vec<i32> {
    let mut result = Vec::new();
    for i in skip..nums.len() {
        let new_num: i32 = nums
            .par_iter()
            .enumerate()
            .filter(|(j, _)| (j + 1) / (i + 1) % 2 != 0) // eliminate all 0's from pattern
            .map(|(j, x)| if (j + 1) / (i + 1) % 4 != 3 { *x } else { -*x }) // negate cases where would be -1 in pattern
            .sum();
        result.push(new_num.abs() % 10);
        if i % 1000 == 0 {
            println!("{} of {}", i, nums.len());
        }
    }
    result
}

fn read_input(s: &str) -> Vec<i32> {
    read_list(s, "")
}

fn perform_n_phases(n: usize, nums: Vec<i32>, skip: usize) -> Vec<i32> {
    let mut result = nums.clone();
    for _ in 0..n {
        println!("starting phase");
        result = perform_phase(result, skip);
        println!("ending phase");
    }
    result
}

fn get_test_input() -> Vec<i32> {
    read_input(include_str!("day16_input.txt"))
}

pub fn perform_fft() -> String {
    let input = get_test_input();
    let result = perform_n_phases(100, input, 0);
    result
        .into_iter()
        .take(8)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn decode_with_fft(input: Vec<i32>) -> Vec<i32> {
    let skip = calculate_skip(&input);
    let nums = std::iter::repeat(input)
        .take(10000)
        .flatten()
        .skip(skip)
        .collect();
    let result = perform_n_phases(100, nums, skip);
    result.into_iter().take(8).collect()
}

fn calculate_skip(nums: &Vec<i32>) -> usize {
    nums.iter().take(7).fold(0, |acc, x| acc * 10 + *x as usize)
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
        let result = perform_phase(list, 0);
        assert_eq!(result, read_input("48226158"));
    }

    #[test]
    fn test_perform_n_phases() {
        let list = read_input("12345678");
        let result = perform_n_phases(4, list, 0);
        assert_eq!(result, read_input("01029498"));
    }
    #[test]
    fn test_large_known_cases() {
        {
            let result = perform_n_phases(100, read_input("80871224585914546619083218645595"), 0)
                .into_iter()
                .take(8)
                .collect::<Vec<_>>();
            assert_eq!(result, read_input("24176176"));
        }
        {
            let result = perform_n_phases(100, read_input("19617804207202209144916044189917"), 0)
                .into_iter()
                .take(8)
                .collect::<Vec<_>>();
            assert_eq!(result, read_input("73745418"));
        }
        {
            let result = perform_n_phases(100, read_input("69317163492948606335995924319873"), 0)
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

    #[test]
    fn test_calculate_skip() {
        assert_eq!(
            calculate_skip(&read_input("03036732577212944063491565474664")),
            0303673
        );
        assert_eq!(
            calculate_skip(&read_input("02935109699940807407585447034323")),
            0293510
        );
        assert_eq!(
            calculate_skip(&read_input("03081770884921959731165446850517")),
            0308177
        );
    }

    #[test]
    #[ignore]
    fn test_decode_with_fft() {
        assert_eq!(
            decode_with_fft(read_input("03036732577212944063491565474664")),
            read_input("84462026")
        );
        assert_eq!(
            decode_with_fft(read_input("02935109699940807407585447034323")),
            read_input("78725270")
        );
        assert_eq!(
            decode_with_fft(read_input("03081770884921959731165446850517")),
            read_input("53553731")
        );
    }
}
