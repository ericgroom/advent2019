use std::fs;
use anyhow::Result;

fn total_fuel_required(modules: Vec<i32>) -> i64 {
    modules.iter().map(|mass| { fuel_required(*mass) as i64 }).sum()
}

pub fn fuel_required(mass: i32) -> i32 {
    mass / 3 - 2
}

fn get_test_input() -> Result<Vec<i32>> {
    let input = fs::read_to_string("./src/day1_input.txt")?;
    Ok(input.split('\n').filter_map(|word| { word.parse::<i32>().ok() }).collect())
}

pub fn get_test_result() -> Result<i64> {
    let modules = get_test_input()?;
    Ok(total_fuel_required(modules))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_test_input() -> Result<()> {
        let modules = get_test_input()?;
        assert_eq!(100, modules.len());
        Ok(())
    }

    #[test]
    fn test_single_known_modules() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }


}