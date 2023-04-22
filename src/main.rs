use generator::*;
use std::time::Instant;

mod generator;
const COHORT_SIZE: usize = 23;
const NUMBER_OF_RUNS: i32 = 1000000;

//TODO: Implement command line parsing for COHORT SIZE and NUMBER OF RUNS

fn main() {
    let now = Instant::now();
    let mut loop_count = 0;
    let mut match_count = 0;
    loop {
        let cohort = Cohort::build(COHORT_SIZE);
        if cohort.check_for_same() {
            match_count += 1;
        }
        loop_count += 1;
        if loop_count == NUMBER_OF_RUNS {
            break;
        }
    }
    let elapsed = now.elapsed();

    let result: f64 = (match_count as f64 / NUMBER_OF_RUNS as f64) * 100.0;
    println!("Simulation took {:.2?}", elapsed);
    println!("{:.2}% chance of match", result);
}
