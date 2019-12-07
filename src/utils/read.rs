extern crate anyhow;

use anyhow::Result;
use std::fs;
use std::str::FromStr;

pub fn read_list<T: FromStr>(input: &str, element_separator: &str) -> Vec<T> {
    input
        .split(element_separator)
        .filter_map(|word| word.parse::<T>().ok())
        .collect()
}

pub fn read_list_from_file<T: FromStr>(path: &str, element_separator: &str) -> Result<Vec<T>> {
    let input = fs::read_to_string(path)?;
    Ok(read_list(&input, element_separator))
}
