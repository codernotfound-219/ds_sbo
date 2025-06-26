#[cfg(test)]
mod test {
    use ds_sbo_rust::greedy_dp::get_action;
    use ds_sbo_rust::greedy_dp::structures::LogHistory;
    use ds_sbo_rust::greedy_dp::Decision;
    use ds_sbo_rust::greedy_dp::{get_creation_deviations, get_insertion_deviations};
    use ds_sbo_rust::resources::problem1::*;
    use ds_sbo_rust::structures::{Batch, BatchSchedule};

    #[test]
    fn insert_job5() {
        let job1 = job1();
        let job6 = job6();

        let tester = job5();

        let mut batch1 = Batch::new(1);
        batch1.insert(job1);
        batch1.insert(job6);

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);

        let insertion_deviations = get_insertion_deviations(&schedule, &tester);
        let creation_deviations = get_creation_deviations(&schedule, &tester);

        let best_action = get_action(&insertion_deviations, &creation_deviations);
        let solution: LogHistory = LogHistory::new(
            6,
            vec![
                Decision::InsertIn { batch_index: 0, job_code: 5 },
                Decision::CreateEnd { job_code: 1 },
            ],
        );

        assert_eq!(best_action, &solution);
    }

    #[test]
    fn insert_job8() {
        let job1 = job1();
        let job5 = job5();
        let job6 = job6();
        let job10 = job10();
        let job7 = job7();
        let tester = job8();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);

        batch1.insert(job5);
        batch1.insert(job6);
        batch2.insert(job7);
        batch3.insert(job1);
        batch3.insert(job10);

        let mut schedule = BatchSchedule::new();

        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);

        let insertion_deviations = get_insertion_deviations(&schedule, &tester);
        let creation_deviations = get_creation_deviations(&schedule, &tester);

        let best_action = get_action(&insertion_deviations, &creation_deviations);
        let solution: LogHistory = LogHistory::new(
            1,
            vec![
                Decision::CreateAt { batch_index: 2, job_code: 8 },
            ],
        );

        assert_eq!(best_action, &solution);
    }

    #[test]
    fn insert_job7() {
        let job1 = job1();
        let job5 = job5();
        let job6 = job6();
        let job10 = job10();
        let tester = job7();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);

        batch1.insert(job5);
        batch1.insert(job6);
        batch2.insert(job1);
        batch2.insert(job10);

        let mut schedule = BatchSchedule::new();

        schedule.insert_end(batch1);
        schedule.insert_end(batch2);

        let insertion_deviations = get_insertion_deviations(&schedule, &tester);
        let creation_deviations = get_creation_deviations(&schedule, &tester);

        let best_action = get_action(&insertion_deviations, &creation_deviations);
        let solution: LogHistory = LogHistory::new(
            2,
            vec![
                Decision::CreateAt { batch_index: 1, job_code: 7 },
            ],
        );

        assert_eq!(best_action, &solution);
    }

    #[test]
    fn insert_job4() {
        let job1 = job1();
        let job5 = job5();
        let job6 = job6();
        let job10 = job10();
        let job7 = job7();
        let job8 = job8();
        let tester = job4();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);

        batch1.insert(job5);
        batch1.insert(job6);
        batch2.insert(job7);
        batch3.insert(job8);
        batch4.insert(job1);
        batch4.insert(job10);

        let mut schedule = BatchSchedule::new();

        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);

        let insertion_deviations = get_insertion_deviations(&schedule, &tester);
        let creation_deviations = get_creation_deviations(&schedule, &tester);

        let best_action = get_action(&insertion_deviations, &creation_deviations);
        let solution: LogHistory = LogHistory::new(
            1,
            vec![
                Decision::InsertIn { batch_index: 2, job_code: 4 },
            ],
        );

        assert_eq!(best_action, &solution);
    }

    #[test]
    fn insert_job3() {
        let job1 = job1();
        let job2 = job2();
        let tester = job3();
        let job4 = job4();
        let job5 = job5();
        let job6 = job6();
        let job7 = job7();
        let job8 = job8();
        let job9 = job9();
        let job10 = job10();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);
        let mut batch5 = Batch::new(5);

        batch1.insert(job5);
        batch1.insert(job6);
        batch2.insert(job7);
        batch3.insert(job8);
        batch3.insert(job4);
        batch4.insert(job9);
        batch4.insert(job1);
        batch5.insert(job2);
        batch5.insert(job10);

        let mut schedule = BatchSchedule::new();

        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);
        schedule.insert_end(batch5);

        let insertion_deviations = get_insertion_deviations(&schedule, &tester);
        let creation_deviations = get_creation_deviations(&schedule, &tester);

        let best_action = get_action(&insertion_deviations, &creation_deviations);
        let solution: LogHistory = LogHistory::new(
            5,
            vec![
                Decision::CreateEnd { job_code: 3 },
            ],
        );

        assert_eq!(best_action, &solution);
    }
}
