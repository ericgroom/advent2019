mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Day 1 fuel required: {}", day1::get_test_result()?);
    println!(
        "Day 1 fuel required with fuel cost: {}",
        day1::get_test_result_with_fuel_cost()?
    );
    println!(
        "Restore gravity assist: {}",
        day2::restore_gravity_assist()?
    );
    println!(
        "Result of 100 * noun + verb: {}",
        day2::noun_and_verb_result()?
    );
    println!(
        "Closest intersection at {} units from central node",
        day3::closest_intersection()?
    );
    println!(
        "Minimal delay intersection at {} units from central node",
        day3::minimal_delay_intersection()?
    );
    println!(
        "Number of valid passwords between 138307-654504: {}",
        day4::valid_passwords_in_input()
    );
    println!(
        "Number of valid passwords using restrictive matching between 138307-654504: {}",
        day4::valid_passwords_in_input_restrictive()
    );
    println!("Diagnostic code: {}", day5::run_diagnostic()?);
    println!("TEST Diagnostic code: {}", day5::run_TEST_diagnostic()?);
    Ok(())
}
