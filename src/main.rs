use generator::*;
mod generator;

fn main() {
    let cohort = Cohort::build_cohort(23);

    println!("{:?}", cohort);

    if cohort.check_for_same() {
        println!("Someone has the same birthday!!")
    } else {
        println!("Same birthday not found");
    }


}
