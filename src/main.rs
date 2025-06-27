use ds_sbo_rust::{greedy_dp::solve::solve, resources::problem1::*, structures::BatchSchedule};

fn main() {
    let job1 = job1();
    let job2 = job2();
    let job3 = job3();
    let job4 = job4();
    let job5 = job5();
    let job6 = job6();
    let job7 = job7();
    let job8 = job8();
    let job9 = job9();
    let job10 = job10();

    let mut list = vec![job1, job2, job3, job4, job5, job6, job7, job8, job9, job10];
    let solution: BatchSchedule = solve(&mut list);

    println!("{}", solution);
}
