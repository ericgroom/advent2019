use std::str::FromStr;

pub fn read_list<T: FromStr>(input: &str, element_separator: &str) -> Vec<T> {
    input
        .split(element_separator)
        .filter_map(|word| word.parse::<T>().ok())
        .collect()
}
