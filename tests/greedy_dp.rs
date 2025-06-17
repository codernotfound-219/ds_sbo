#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, BatchSchedule, Job};
    use ds_sbo_rust::greedy_dp::{locate_eligible_batch, solve};
    use ds_sbo_rust::resources::problem1::*;

    #[test]
    fn test_step3() {
        let job1 = job1();
        let job5 = job5();
        let job6 = job6();
        let job10 = job10();
        let job7 = job7();
        let job8 = job8();

        let mut list: Vec<Job> = vec![job1.clone(), job5.clone(), job6.clone(), job10.clone(), job7.clone(), job8.clone()];
        let output = solve(&mut list);
        
        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);
        let mut batch5 = Batch::new(5);
        let mut schedule = BatchSchedule::new();

        batch1.insert(job5);
        batch2.insert(job7);
        batch3.insert(job8);
        batch4.insert(job6);
        batch4.insert(job1);
        batch5.insert(job10);
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);
        schedule.insert_end(batch5);

        println!("{}", output);
        assert_eq!(output, schedule);
    }

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
        let output = solve(&mut list);

        let answer = BatchSchedule::new();
    }
}
