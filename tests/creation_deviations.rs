#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, BatchSchedule};
    use ds_sbo_rust::greedy_dp::deviation_calculator::creation::get_creation_deviations;
    use ds_sbo_rust::greedy_dp::{Decision, LogHistory};
    use ds_sbo_rust::resources::problem1::*;

    #[test]
    fn test_creation_for_job4() {
        let job1 = job1();
        let job5 = job5();
        let job6 = job6();
        let job7 = job7();
        let job8 = job8();
        let job10 = job10();
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

        let set_m = get_creation_deviations(&schedule, &tester);
        let solution: Vec<LogHistory> = vec![
            LogHistory::new(
                -34,
                vec![Decision::CreateAt {
                    batch_index: 0,
                    job_code: 4,
                }],
            ),
            LogHistory::new(
                -3,
                vec![Decision::CreateAt {
                    batch_index: 1,
                    job_code: 4,
                }],
            ),
            LogHistory::new(
                -2,
                vec![Decision::CreateAt {
                    batch_index: 2,
                    job_code: 4,
                }],
            ),
            LogHistory::new(
                -2,
                vec![Decision::CreateAt {
                    batch_index: 3,
                    job_code: 4,
                }],
            ),
            LogHistory::new(
                -9,
                vec![Decision::CreateEnd {
                    job_code: 4,
                }],
            ),
        ];

        assert_eq!(set_m, solution);
    }

    #[test]
    fn test_creation_for_job3() {
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
        batch4.insert(job1);
        batch4.insert(job9);
        batch5.insert(job10);
        batch5.insert(job2);

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);
        schedule.insert_end(batch5);

        let set_m = get_creation_deviations(&schedule, &tester);
        let solution: Vec<LogHistory> = vec![
            LogHistory::new(
                -88,
                vec![Decision::CreateAt {
                    batch_index: 0,
                    job_code: 3,
                }],
            ),
            LogHistory::new(
                -40,
                vec![Decision::CreateAt {
                    batch_index: 1,
                    job_code: 3,
                }],
            ),
            LogHistory::new(
                -28,
                vec![Decision::CreateAt {
                    batch_index: 2,
                    job_code: 3,
                }],
            ),
            LogHistory::new(
                -13,
                vec![Decision::CreateAt {
                    batch_index: 3,
                    job_code: 3,
                }],
            ),
            LogHistory::new(
                -6,
                vec![Decision::CreateAt {
                    batch_index: 4,
                    job_code: 3,
                }],
            ),
            LogHistory::new(
                5,
                vec![Decision::CreateEnd {
                    job_code: 3,
                }],
            ),
        ];


        assert_eq!(set_m, solution);
    }
}
