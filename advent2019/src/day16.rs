// list of ints as input
// each phase, a new list is generated with the same length as the previous
// result of phase = summation of elements * pattern, only one's digit is preserved

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
}
