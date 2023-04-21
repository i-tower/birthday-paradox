use generator::*;
mod generator;
const COHORT_SIZE: usize = 23;
fn main() {
    let cohort = Cohort::build(23);

    println!("{:?}", cohort);

    if cohort.check_for_same() {
        println!("Someone has the same birthday!!")
    } else {
        println!("Same birthday not found");
    }

    let runs = 100;
    let mut loop_count = 0;
    let mut match_count = 0;
    loop {
        let cohort = Cohort::build(COHORT_SIZE);
        if cohort.check_for_same() {
            match_count += 1;
        }
        loop_count += 1;
        if loop_count == runs {
            break
        }
    }

    let result: f64  = (match_count / runs) * 100;

    println!("{}% chance of match", result);


}
