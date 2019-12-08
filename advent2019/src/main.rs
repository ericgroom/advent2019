mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

extern crate anyhow;

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
    println!("TEST Diagnostic code: {}", day5::run_test_diagnostic()?);
    println!(
        "Number of orbits: {}",
        day6::find_direct_and_indirect_orbits()?
    );
    println!("Distance to Santa: {}", day6::distance_to_santa()?);
    println!(
        "Max thruster signal: {}",
        day7::find_highest_thruster_signal()?
    );
    println!(
        "Max thruster signal in feedback loop: {}",
        day7::find_feedback_loop_max()?
    );

    Ok(())
}