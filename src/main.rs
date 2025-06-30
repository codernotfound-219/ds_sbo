use ds_sbo_rust::{
    greedy_dp::solve::solve,
    resources::{problem1::problem1, problem2::problem2, problem3::problem3},
    structures::BatchSchedule, tardiness_calculator::get_tardiness,
};

fn main() {

    let mut problem1 = problem1();
    let solution1: BatchSchedule = solve(&mut problem1);
    let tardiness1 = get_tardiness(&solution1);
    println!("{}", solution1);
    println!("Total Tardiness: {}", tardiness1);

    // let mut problem2 = problem2();
    // let solution2: BatchSchedule = solve(&mut problem2);
    // let tardiness2 = get_tardiness(&solution2);
    // println!("{}", solution2);
    // println!("Total Tardiness: {}", tardiness2);

    // let mut problem3 = problem3();
    // let solution3: BatchSchedule = solve(&mut problem3);
    // let tardiness3 = get_tardiness(&solution3);
    // println!("{}", solution3);
    // println!("Total Tardiness: {}", tardiness3);
}
