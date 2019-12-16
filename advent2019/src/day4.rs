use crate::utils::math::digits_of;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

pub fn find_valid_passwords(range: Range<i32>, use_restrictive_matching: bool) -> HashSet<i32> {
    let mut result_set = HashSet::new();
    for password in range {
        if is_valid_password(password, use_restrictive_matching) {
            result_set.insert(password);
        }
    }
    result_set
}

fn is_sorted(num_list: &Vec<i32>) -> bool {
    if num_list.len() <= 1 {
        return true;
    }
    let mut previous = num_list[0];
    for num in num_list.into_iter().cloned() {
        if num < previous {
            return false;
        }
        previous = num;
    }
    return true;
}

fn has_identical_adjacent_elements(num_list: &Vec<i32>) -> bool {
    if num_list.len() <= 1 {
        return false;
    }
    let mut previous = num_list[0];
    for num in num_list[1..].into_iter().cloned() {
        if num == previous {
            return true;
        }
        previous = num;
    }
    return false;
}

fn has_identical_adjacent_elements_of_length_two(num_list: &Vec<i32>) -> bool {
    if num_list.len() <= 1 {
        return false;
    }
    let mut digit_occurrences: HashMap<i32, i32> = HashMap::new();
    for num in num_list.into_iter().cloned() {
        if digit_occurrences.contains_key(&num) {
            let old_value = digit_occurrences[&num];
            digit_occurrences.insert(num, old_value + 1);
        } else {
            digit_occurrences.insert(num, 1);
        }
    }
    digit_occurrences.values().any(|x| *x == 2)
}

pub fn is_valid_password(password: i32, use_restrictive_matching: bool) -> bool {
    let digits = digits_of(password);
    if digits.len() != 6 {
        return false;
    }
    if !is_sorted(&digits) {
        return false;
    }
    if !has_identical_adjacent_elements(&digits) {
        return false;
    }
    if use_restrictive_matching && !has_identical_adjacent_elements_of_length_two(&digits) {
        return false;
    }
    true
}

pub fn valid_passwords_in_input() -> i32 {
    let input_range = 138_307..654_504;
    find_valid_passwords(input_range, false).len() as i32
}

pub fn valid_passwords_in_input_restrictive() -> i32 {
    let input_range = 138_307..654_504;
    find_valid_passwords(input_range, true).len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_digits() {
        assert_eq!(digits_of(0), vec![]);
        assert_eq!(digits_of(123456), vec![1, 2, 3, 4, 5, 6])
    }

    #[test]
    fn test_is_sorted() {
        assert_eq!(is_sorted(&digits_of(112233)), true);
        assert_eq!(is_sorted(&digits_of(112232)), false);
    }

    #[test]
    fn test_has_identical_adjacent_elements() {
        assert_eq!(has_identical_adjacent_elements(&digits_of(111111)), true);
        assert_eq!(has_identical_adjacent_elements(&digits_of(123456)), false);
        assert_eq!(has_identical_adjacent_elements(&digits_of(123345)), true);
    }

    #[test]
    fn test_is_valid_password() {
        assert_eq!(is_valid_password(111111, false), true);
        assert_eq!(is_valid_password(223450, false), false);
        assert_eq!(is_valid_password(123789, false), false);
    }

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(valid_passwords_in_input(), 1855);
    }

    #[test]
    fn test_has_identical_adjacent_elements_of_length_two() {
        assert_eq!(
            has_identical_adjacent_elements_of_length_two(&digits_of(112233)),
            true
        );
        assert_eq!(
            has_identical_adjacent_elements_of_length_two(&digits_of(123444)),
            false
        );
        assert_eq!(
            has_identical_adjacent_elements_of_length_two(&digits_of(111122)),
            true
        );
    }

    #[test]
    fn test_correct_answer_part_2() {
        assert_eq!(valid_passwords_in_input_restrictive(), 1253);
    }
}
