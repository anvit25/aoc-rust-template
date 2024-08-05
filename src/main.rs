pub mod solutions;

use solutions::{run_all_till, run_day};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    run_all_till(1)?;
    run_day(1)?;
    println!("\nTotal Time: {}ms", now.elapsed().as_millis());
    Ok(())
}
