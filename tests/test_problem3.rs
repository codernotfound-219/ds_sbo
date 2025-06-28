#[cfg(test)]
mod test {
    use ds_sbo_rust::greedy_dp::solve::solve;
    use ds_sbo_rust::resources::problem3;
    use ds_sbo_rust::structures::BatchSchedule;

    #[test]
    fn check_prob3_solution() {
        let og_solution: BatchSchedule = problem3::solution();
        let mut problem3 = problem3::problem3();
        let solution3: BatchSchedule = solve(&mut problem3);

        assert_eq!(og_solution, solution3);
    }
}
