mod day1;
mod day2;
mod day3;

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
    Ok(())
}
