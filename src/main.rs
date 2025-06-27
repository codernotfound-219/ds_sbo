use ds_sbo_rust::{
    greedy_dp::solve::solve,
    resources::problem3,
    structures::BatchSchedule,
};

fn main() {
    // let p1_job1 = problem1::job1();
    // let p1_job2 = problem1::job2();
    // let p1_job3 = problem1::job3();
    // let p1_job4 = problem1::job4();
    // let p1_job5 = problem1::job5();
    // let p1_job6 = problem1::job6();
    // let p1_job7 = problem1::job7();
    // let p1_job8 = problem1::job8();
    // let p1_job9 = problem1::job9();
    // let p1_job10 = problem1::job10();
    //
    // let mut list1 = vec![
    //     p1_job1, p1_job2, p1_job3, p1_job4, p1_job5, p1_job6, p1_job7, p1_job8, p1_job9, p1_job10,
    // ];
    // let solution1: BatchSchedule = solve(&mut list1);
    // println!("{}", solution1);
    //
    //
    // let p2_job1 = problem2::job1();
    // let p2_job2 = problem2::job2();
    // let p2_job3 = problem2::job3();
    // let p2_job4 = problem2::job4();
    // let p2_job5 = problem2::job5();
    // let p2_job6 = problem2::job6();
    // let p2_job7 = problem2::job7();
    // let p2_job8 = problem2::job8();
    // let p2_job9 = problem2::job9();
    // let p2_job10 = problem2::job10();
    // let mut list2 = vec![
    //     p2_job1, p2_job2, p2_job3, p2_job4, p2_job5, p2_job6, p2_job7, p2_job8, p2_job9, p2_job10,
    // ];
    // let solution2: BatchSchedule = solve(&mut list2);
    // println!("{}", solution2);
    //

    let p3_job1 = problem3::job1();
    let p3_job2 = problem3::job2();
    let p3_job3 = problem3::job3();
    let p3_job4 = problem3::job4();
    let p3_job5 = problem3::job5();
    let p3_job6 = problem3::job6();
    let p3_job7 = problem3::job7();
    let p3_job8 = problem3::job8();
    let p3_job9 = problem3::job9();
    let p3_job10 = problem3::job10();
    let p3_job11 = problem3::job11();
    let p3_job12 = problem3::job12();
    let p3_job13 = problem3::job13();
    let p3_job14 = problem3::job14();
    let p3_job15 = problem3::job15();
    let p3_job16 = problem3::job16();
    let p3_job17 = problem3::job17();
    let p3_job18 = problem3::job18();
    let p3_job19 = problem3::job19();
    let p3_job20 = problem3::job20();
    let p3_job21 = problem3::job21();
    let p3_job22 = problem3::job22();
    let p3_job23 = problem3::job23();
    let p3_job24 = problem3::job24();
    let p3_job25 = problem3::job25();
    let mut list3 = vec![
        p3_job1, p3_job2, p3_job3, p3_job4, p3_job5, p3_job6, p3_job7, p3_job8, p3_job9, p3_job10,
        p3_job11, p3_job12, p3_job13, p3_job14, p3_job15, p3_job16, p3_job17, p3_job18, p3_job19,
        p3_job20, p3_job21, p3_job22, p3_job23, p3_job24, p3_job25,
    ];
    let solution: BatchSchedule = solve(&mut list3);
    println!("{}", solution);
}
