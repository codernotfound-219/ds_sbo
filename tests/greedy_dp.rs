#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, BatchSchedule, Job};
    use ds_sbo_rust::greedy_dp::solve;
    use ds_sbo_rust::resources::problem1::*;

    #[test]
    fn test_problem1() {
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

        let mut list: Vec<Job> = vec![job1, job2, job3, job4, job5, job6, job7, job8, job9, job10];
    }
}
