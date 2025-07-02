use ds_sbo_rust::{
    marb_heuristic::solve::solve as marb_heuristic_solver,
    greedy_dp::solve::solve as greedy_dp_solver,
    resources:: problem3::{problem3, solution},
    structures::BatchSchedule, tardiness_calculator::get_tardiness,
};

fn main() {

    // let mut problem1 = problem1();
    // let solution1: BatchSchedule = solve(&mut problem1);
    // println!();
    // println!("Solution to problem3 (25 jobs): ");
    // println!();
    // println!("===================================");
    // println!("{}", solution1);
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

    let solution3: BatchSchedule = greedy_dp_solver(&mut problem3());
    println!();
    println!("Solution to problem3 (25 jobs): ");
    println!();
    println!("{}", solution3);
    println!("===================================");
    let tardiness3 = get_tardiness(&solution3);
    println!("===================================");
    println!("Total Tardiness: {}", tardiness3);

    // let mut problem3 = problem3();
    // let solution3: BatchSchedule = marb_heuristic::solver(&mut problem3);
    // println!("{}", solution3);
    // println!("===================================");
    // let tardiness3 = get_tardiness(&solution3);
    // println!("===================================");
    // println!("Total Tardiness: {}", tardiness3);

    let list: BatchSchedule = marb_heuristic_solver(&mut problem3());
    let solution: BatchSchedule = solution();

    assert_eq!(list, solution, "INVALID");
    println!("{}", list);
}
