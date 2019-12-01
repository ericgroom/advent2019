mod day1;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Day 1 fuel required: {}", day1::get_test_result()?);
    Ok(())
}
