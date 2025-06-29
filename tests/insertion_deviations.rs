#[cfg(test)]
mod test {
    use ds_sbo_rust::greedy_dp::get_insertion_deviations;
    use ds_sbo_rust::structures::{Batch, BatchSchedule, Job};
    use ds_sbo_rust::greedy_dp::structures::LogHistory;
    use ds_sbo_rust::greedy_dp::Decision;
    use ds_sbo_rust::resources::problem1::*;

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

        let set_m = get_insertion_deviations(&schedule, &tester);
        let solution: Vec<LogHistory> = vec![LogHistory::new(
            6,
            vec![
                Decision::InsertIn {
                    batch_index: 0,
                    job_code: 5,
                },
                Decision::CreateEnd { job_code: 1 },
            ],
        )];
        assert_eq!(set_m, solution);
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

        let set_m = get_insertion_deviations(&schedule, &tester);
        let solution: Vec<LogHistory> = vec![
            LogHistory::new(
                -2,
                vec![
                    Decision::InsertIn {
                        batch_index: 0,
                        job_code: 8,
                    },
                    Decision::InsertIn {
                        batch_index: 2,
                        job_code: 6,
                    },
                    Decision::CreateEnd { job_code: 10 },
                ],
            ),
            LogHistory::new(
                -1,
                vec![Decision::InsertIn {
                    batch_index: 1,
                    job_code: 8,
                }],
            ),
            LogHistory::new(
                -2,
                vec![
                    Decision::InsertIn {
                        batch_index: 2,
                        job_code: 8,
                    },
                    Decision::CreateEnd { job_code: 10 },
                ],
            ),
        ];

        assert_eq!(set_m, solution);
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

        let set_m = get_insertion_deviations(&schedule, &tester);
        let solution: Vec<LogHistory> = vec![
            LogHistory::new(
                -3,
                vec![
                    Decision::InsertIn {
                        batch_index: 0,
                        job_code: 7,
                    },
                    Decision::InsertIn {
                        batch_index: 1,
                        job_code: 6,
                    },
                    Decision::CreateEnd { job_code: 10 },
                ],
            ),
            LogHistory::new(
                -6,
                vec![
                    Decision::InsertIn {
                        batch_index: 1,
                        job_code: 7,
                    },
                    Decision::CreateEnd { job_code: 10 },
                ],
            ),
        ];
        assert_eq!(set_m, solution);
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

        let set_m = get_insertion_deviations(&schedule, &tester);
        let solution: Vec<LogHistory> = vec![
            LogHistory::new(
                -7,
                vec![
                    Decision::InsertIn {
                        batch_index: 0,
                        job_code: 4,
                    },
                    Decision::InsertIn {
                        batch_index: 3,
                        job_code: 6,
                    },
                    Decision::CreateEnd { job_code: 10 },
                ],
            ),
            LogHistory::new(
                -1,
                vec![Decision::InsertIn {
                    batch_index: 1,
                    job_code: 4,
                }],
            ),
            LogHistory::new(
                1,
                vec![Decision::InsertIn {
                    batch_index: 2,
                    job_code: 4,
                }],
            ),
            LogHistory::new(
                -6,
                vec![
                    Decision::InsertIn {
                        batch_index: 3,
                        job_code: 4,
                    },
                    Decision::CreateEnd { job_code: 10 },
                ],
            ),
        ];
        assert_eq!(set_m, solution);
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

        let set_m = get_insertion_deviations(&schedule, &tester);
        let solution: Vec<LogHistory> = vec![
            LogHistory {
                deviation: -2147483648,
                actions: vec![Decision::NotPossible],
            },
            LogHistory {
                deviation: -36,
                actions: vec![Decision::InsertIn {
                    batch_index: 1,
                    job_code: 3,
                }],
            },
            LogHistory {
                deviation: -2147483648,
                actions: vec![Decision::NotPossible],
            },
            LogHistory {
                deviation: -2147483648,
                actions: vec![Decision::NotPossible],
            },
            LogHistory {
                deviation: -2147483648,
                actions: vec![Decision::NotPossible],
            },
        ];

        assert_eq!(set_m, solution);
    }

    #[test]
    fn self_test() {
        let job1 = Job {
            code: 1,
            release_date: 9,
            processing_time: 6,
            due_date: 21,
            size: 10,
        };
        let job2 = Job {
            code: 2,
            release_date: 12,
            processing_time: 4,
            due_date: 32,
            size: 9,
        };
        let job3 = Job {
            code: 3,
            release_date: 13,
            processing_time: 5,
            due_date: 42,
            size: 9,
        };
        let tester = Job {
            code: 4,
            release_date: 14,
            processing_time: 4,
            due_date: 18,
            size: 9,
        };
        
        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);

        batch1.insert(job1);
        batch1.insert(job2);
        batch2.insert(job3);

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);

        let _insertion_list = get_insertion_deviations(&schedule, &tester);
        assert_eq!(true, false);
    }
}
