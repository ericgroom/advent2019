mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn main() {
    println!("Day 1 fuel required: {}", day1::get_test_result());
    println!(
        "Day 1 fuel required with fuel cost: {}",
        day1::get_test_result_with_fuel_cost()
    );
    println!("Restore gravity assist: {}", day2::restore_gravity_assist());
    println!(
        "Result of 100 * noun + verb: {}",
        day2::noun_and_verb_result()
    );
    println!(
        "Closest intersection at {} units from central node",
        day3::closest_intersection()
    );
    println!(
        "Minimal delay intersection at {} units from central node",
        day3::minimal_delay_intersection()
    );
    println!(
        "Number of valid passwords between 138307-654504: {}",
        day4::valid_passwords_in_input()
    );
    println!(
        "Number of valid passwords using restrictive matching between 138307-654504: {}",
        day4::valid_passwords_in_input_restrictive()
    );
    println!("Diagnostic code: {}", day5::run_diagnostic());
    println!("TEST Diagnostic code: {}", day5::run_test_diagnostic());
    println!(
        "Number of orbits: {}",
        day6::find_direct_and_indirect_orbits()
    );
    println!("Distance to Santa: {}", day6::distance_to_santa());
    println!(
        "Max thruster signal: {}",
        day7::find_highest_thruster_signal()
    );
    println!(
        "Max thruster signal in feedback loop: {}",
        day7::find_feedback_loop_max()
    );
    println!("Image corruption check: {}", day8::ensure_no_corruption());
    print!("{}", day8::display_password());
    println!("BOOST keycode: {}", day9::run_boost_diagnostic());
    println!(
        "Ceres distress signal coords: {}",
        day9::get_distress_signal_coords()
    );
    println!(
        "Best asteroid has visibility: {}",
        day10::find_ideal_asteroid()
    );
    println!("200th: {}", day10::get_two_hundredth());
    println!("Hull coverate: {}", day11::get_coverage());
    println!("{}", day11::get_registration_identifier());
    println!("Total energy: {}", day12::get_moons_simulation());
    println!("Repetition at: {}", day12::does_it_repeat());
    println!("Number of blocks rendered: {}", day13::count_blocks());
    println!("Score: {}", day13::play_game());
    println!("Number of ORE needed: {}", day14::find_ore_cost()); // 938867 too high, 832696 too low
}
