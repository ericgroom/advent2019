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

fn resolve_image_layers(layers: Vec<Vec<i32>>) -> Vec<i32> {
    let area = layers[0].len();
    let mut result = Vec::with_capacity(area);
    for i in 0..area {
        for layer in &layers {
            let pixel = layer[i];
            match pixel {
                2 => continue,
                x if x == 1 || x == 0 => {
                    result.push(x);
                    break;
                }
                _ => panic!("Value other than 0, 1, 2"),
            }
        }
    }
    result
}

fn render_image(image: Vec<i32>, row_width: usize) -> String {
    let mut result = String::new();
    for row in image.chunks(row_width) {
        for pixel in row {
            let c = match pixel {
                0 => '█',
                1 => ' ',
                2 => 't',
                _ => panic!("pixel other than 0, 1, 2"),
            };
            result.push(c)
        }
        result.push('\n')
    }
    result
}

pub fn display_password() -> String {
    let width = 25;
    let height = 6;
    let input = get_test_input();
    let layers = split_into_layers(input, width * height);
    let image = resolve_image_layers(layers);
    render_image(image, width)
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

    #[test]
    fn test_resolve_layers() {
        let layers = vec![
            vec![0, 2, 2, 2],
            vec![1, 1, 2, 2],
            vec![2, 2, 1, 2],
            vec![0, 0, 0, 0],
        ];
        let final_image = resolve_image_layers(layers);
        assert_eq!(final_image, vec![0, 1, 1, 0]);
    }

    #[test]
    fn test_render_image() {
        let image = vec![0, 1, 2, 2, 1, 0];
        assert_eq!(render_image(image, 3), String::from("█ t\nt █\n"));
    }

    #[test]
    fn test_correct_answer_part_2() {
        let image = display_password();
        assert_eq!(
            image,
            "█  ██ ██ ██  ██ ██ █ ██ █\n ██ █ █ ██ ██ █ █ ██ ██ █\n ████  ███ ████  ███    █\n █  █ █ ██ ████ █ ██ ██ █\n ██ █ █ ██ ██ █ █ ██ ██ █\n█   █ ██ ██  ██ ██ █ ██ █\n"
        )
    }
}
