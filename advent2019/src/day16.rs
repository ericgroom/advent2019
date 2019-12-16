// list of ints as input
// each phase, a new list is generated with the same length as the previous
// result of phase = summation of elements * pattern, only one's digit is preserved
use std::cell::Cell;

struct PatternIter {
    subsequnce_length: usize,
    subsequence_index: Cell<usize>,
    element_cycler: Cell<usize>,
}

impl PatternIter {
    fn new(subsequnce_length: usize) -> PatternIter {
        PatternIter {
            subsequnce_length: subsequnce_length,
            subsequence_index: Cell::default(),
            element_cycler: Cell::default(),
        }
    }
}

impl Iterator for PatternIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut element_cycler = self.element_cycler.get_mut();
        let mut subsequence_index = self.subsequence_index.get_mut();
        let num = match element_cycler {
            0 => 0,
            1 => 1,
            2 => 0,
            3 => -1,
            _ => panic!("unexpected element cycler"),
        };

        if *subsequence_index % self.subsequnce_length == 0 {
            println!("incr");
            *element_cycler = (*element_cycler + 1) % 4;
        }
        *subsequence_index += 1;

        println!(
            "num: {}, cycler: {}, sub_idx: {}, sub_len: {}",
            num, *element_cycler, *subsequence_index, self.subsequnce_length
        );
        Some(num)
    }
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
        {
            let iter = PatternIter::new(2);
            let result: Vec<_> = iter.skip(1).take(8).collect();
            assert_eq!(result, vec![0, 1, 1, 0, 0, -1, -1, 0]);
        }
        {
            let iter = PatternIter::new(3);
            let result: Vec<_> = iter.skip(1).take(8).collect();
            assert_eq!(result, vec![0, 1, 1, 0, 0, -1, -1, 0]);
        }
    }
}
