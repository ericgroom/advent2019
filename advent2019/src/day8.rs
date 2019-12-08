use crate::utils::read::read_list;

fn split_into_layers(image_data: Vec<i32>, area: usize) -> Vec<Vec<i32>> {
    image_data.chunks(area).map(|s| s.to_vec()).collect()
}

fn count_digit(list: &Vec<i32>, digit: i32) -> usize {
    list.into_iter()
        .filter(|x| **x == digit)
        .collect::<Vec<&i32>>()
        .len()
}

fn find_layer_with_fewest_zeros(image_data: Vec<i32>, area: usize) -> Vec<i32> {
    let layers = split_into_layers(image_data, area);
    let mut min_layer = layers[0].clone();
    let mut min_zero_count = count_digit(&min_layer, 0) as i32;
    for layer in layers.into_iter() {
        let zero_count = count_digit(&layer, 0) as i32;
        if zero_count < min_zero_count {
            min_zero_count = zero_count;
            min_layer = layer;
        }
    }
    min_layer.to_owned()
}

fn get_test_input() -> Vec<i32> {
    read_list(include_str!("./day8_input.txt"), "")
}

pub fn ensure_no_corruption() -> usize {
    let input = get_test_input();
    let width = 25;
    let height = 6;
    assert_eq!(input.len() % (25 * 6), 0);
    let layer_with_min_zeros = find_layer_with_fewest_zeros(input, width * height);
    count_digit(&layer_with_min_zeros, 1) * count_digit(&layer_with_min_zeros, 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_digit() {
        assert_eq!(count_digit(&vec![], 0), 0);
        assert_eq!(count_digit(&vec![1, 2, 3, 0, 2], 0), 1);
        assert_eq!(count_digit(&vec![1, 2, 3, 0, 2], 2), 2);
    }

    #[test]
    fn test_split_into_layers() {
        assert_eq!(
            split_into_layers(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2], 6),
            vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]]
        );
    }

    #[test]
    fn test_find_layer_with_fewest_zeros() {
        assert_eq!(
            find_layer_with_fewest_zeros(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2], 6),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(ensure_no_corruption(), 1463);
    }
}
