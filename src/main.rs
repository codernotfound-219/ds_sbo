use ds_sbo_rust::{
    marb_heuristic,
    resources::problem3::{problem3, solution},
    structures::{BatchSchedule, Job}, tardiness_calculator::get_tardiness,
};

fn main() {

    // let mut problem1 = problem1();
    // let solution1: BatchSchedule = solve(&mut problem1);
    // println!("{}", solution1);
    // println!("===================================");
    // let tardiness1 = get_tardiness(&solution1);
    // println!("===================================");
    // println!("Total Tardiness: {}", tardiness1);

    // let mut problem2 = problem2();
    // let solution2: BatchSchedule = solve(&mut problem2);
    // println!("{}", solution2);
    // println!("===================================");
    // let tardiness2 = get_tardiness(&solution2);
    // println!("===================================");
    // println!("Total Tardiness: {}", tardiness2);

    // let mut problem3 = problem3();
    // let solution3: BatchSchedule = marb_heuristic::solver(&mut problem3);
    // println!("{}", solution3);
    // println!("===================================");
    // let tardiness3 = get_tardiness(&solution3);
    // println!("===================================");
    // println!("Total Tardiness: {}", tardiness3);

    let list: BatchSchedule = marb_heuristic::solve::solve(&mut problem3());
    let solution: BatchSchedule = solution();

    assert_eq!(list, solution, "INVALID");
    println!("{}", list);
}
