use crate::utils::read::read_list;

fn total_fuel_required(modules: Vec<i32>) -> i64 {
    modules.iter().map(|mass| fuel_required(*mass) as i64).sum()
}

fn total_fuel_required_with_fuel_cost(modules: Vec<i32>) -> i64 {
    modules
        .iter()
        .map(|mass| fuel_required(*mass) as i64)
        .map(|fuel| fuel + calculate_fuel_cost(fuel))
        .sum()
}

pub fn calculate_fuel_cost(fuel: i64) -> i64 {
    let fuel_needed_for_fuel = fuel_required(fuel as i32) as i64;
    if fuel_needed_for_fuel <= 0 {
        return 0;
    }
    fuel_needed_for_fuel + calculate_fuel_cost(fuel_needed_for_fuel)
}

pub fn fuel_required(mass: i32) -> i32 {
    mass / 3 - 2
}

fn get_test_input() -> Vec<i32> {
    read_list(include_str!("./day1_input.txt"), "\n")
}

pub fn get_test_result() -> i64 {
    let modules = get_test_input();
    total_fuel_required(modules)
}

pub fn get_test_result_with_fuel_cost() -> i64 {
    let modules = get_test_input();
    total_fuel_required_with_fuel_cost(modules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_test_input() {
        let modules = get_test_input();
        assert_eq!(100, modules.len());
    }

    #[test]
    fn test_single_known_modules() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }

    #[test]
    fn test_calculate_fuel_cost() {
        assert_eq!(calculate_fuel_cost(2), 0);
        assert_eq!(calculate_fuel_cost(654), 966 - 654);
        assert_eq!(calculate_fuel_cost(33583), 50346 - 33583);
    }

    #[test]
    fn test_total_with_fuel_cost() {
        assert_eq!(total_fuel_required_with_fuel_cost(vec!(14)), 2);
        assert_eq!(total_fuel_required_with_fuel_cost(vec!(1969)), 966);
        assert_eq!(total_fuel_required_with_fuel_cost(vec!(100756)), 50346);
    }

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(get_test_result(), 3223398);
    }

    #[test]
    fn test_correct_answer_part_2() {
        assert_eq!(get_test_result_with_fuel_cost(), 4832253);
    }
}
