use aoc_utils::io::download_input;
use std::error::Error;
use std::time::Instant;

pub mod day1;

pub fn run_day(day: u8) -> Result<(), Box<dyn Error>> {
    download_input(day, 2016)?;
    match day {
        1 => println!("Day 1a: {}, Day 1b: {}", day1::day1a(), day1::day1b()),
        _ => {
            return Err("Day not implemented".into());
        }
    };
    Ok(())
}

pub fn run_all_till(n: u8) -> Result<(), Box<dyn Error>> {
    for day in 1..n + 1 {
        let now = Instant::now();
        run_day(day)?;
        println!("Time: {}ms", now.elapsed().as_millis());
    }
    Ok(())
}
