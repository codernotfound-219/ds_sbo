#[cfg(test)]
mod tests {
    use ds_sbo_rust::core::{Batch, BatchSchedule, Job};
    use ds_sbo_rust::algorithm2::locate_eligible_batch;
    use ds_sbo_rust::resources::problem2::*;

    #[test]
    fn check_index() {
        let job1 = job1();
        let job2 = job2();
        let job4 = job4();
        let job5 = job5();
        let job6 = job6();
        let job7 = job7();
        let job8 = job8();
        let job10 = job10();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);
        let mut batch5 = Batch::new(5);
        let mut batch6 = Batch::new(6);

        batch1.insert_begin(job10);
        batch1.insert_begin(job2);
        batch2.insert_begin(job8);
        batch3.insert_begin(job1);
        batch4.insert_begin(job4);
        batch5.insert_begin(job6);
        batch5.insert_begin(job5);
        batch6.insert_begin(job7);

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);
        schedule.insert_end(batch5);
        schedule.insert_end(batch6);

        let tester = job9();
        let result_index = locate_eligible_batch(&schedule, tester.due_date);

        assert_eq!(result_index, Some(3));
    }
    // Shared test data as functions to create fresh instances
    #[test]
    fn test_comparison_basic() {
        let job1 = job1();
        let job6 = job6();

        let result = comparison(&job1, &job6, 0);

        assert_eq!(result.priority, Priority::Current);
        assert_eq!(result.status, Status::Pass);
    }

    #[test]
    fn test_comparison_deadline_violation() {
        let late_job = job7();
        let early_job = job2();

        let result = comparison(&late_job, &early_job, 0);

        // should be Fail, as job7 will be delayed by 3.
        match result.status {
            Status::Fail(lateness) => assert_eq!(lateness, 3),
            Status::Pass => panic!("Expected failure status"),
        }

        match result.priority {
            Priority::Head => (),
            Priority::Current => panic!("Expected Head to be the priority"),
        }
    }

    #[test]
    fn test_size_check() {
        let job = Job::new(1, 10, 20, 5, 3);
        let mut batch = Batch::new(1);
        batch.insert_begin(Job::new(2, 5, 15, 3, 4));

        assert!(size_check(10, &batch, &job)); // 4 + 3 <= 10
        assert!(!size_check(6, &batch, &job)); // 4 + 3 > 6
    }

    // #[test]
    // fn final_check() {
    //     let job1 = Job::new(1, 1, 29, 9, 9);
    //     let job2 = Job::new(2, 16, 31, 4, 7);
    //     let job3 = Job::new(3, 20, 42, 5, 9);
    //     let job4 = Job::new(4, 14, 22, 3, 6);
    //     let job5 = Job::new(5, 5, 22, 6, 6);
    //     let job6 = Job::new(6, 4, 27, 9, 9);
    //     let job7 = Job::new(7, 13, 17, 1, 8);
    //     let job8 = Job::new(8, 13, 22, 4, 9);
    //     let job9 = Job::new(9, 18, 28, 8, 6);
    //     let job10 = Job::new(10, 6, 38, 4, 7);
    //
    //     let mut list = vec![&job1, &job2, &job3, &job4, &job5, &job6, &job7, &job8, &job9, &job10];
    //
    //     let result = solve(&mut list);
    //     let lateness = 1;
    //
    //     let mut answer = BatchSchedule::new();
    //     let mut batch1 = Batch::new(1);
    //     let mut batch2 = Batch::new(2);
    //     let mut batch3 = Batch::new(3);
    //     let mut batch4 = Batch::new(4);
    //     let mut batch5 = Batch::new(5);
    //     let mut batch6 = Batch::new(6);
    //
    //     batch1.insert_begin(&job5);
    //     batch1.insert_end(&job6);
    //     batch2.insert_begin(&job7);
    //     batch3.insert_begin(&job8);
    //     batch3.insert_end(&job4);
    //     batch4.insert_begin(&job9);
    //     batch4.insert_end(&job1);
    //     batch5.insert_begin(&job2);
    //     batch5.insert_end(&job10);
    //     batch6.insert_begin(&job3);
    //
    //     answer.insert_end(batch6);
    //     answer.insert_end(batch5);
    //     answer.insert_end(batch4);
    //     answer.insert_end(batch3);
    //     answer.insert_end(batch2);
    //     answer.insert_end(batch1);
    //
    // }
}
