#[cfg(test)]
mod test {
    use ds_sbo_rust::structures::{Job, BatchSchedule, Batch};
    use ds_sbo_rust::greedy_dp::{get_insertion_deviations, LogHistory};

    #[test]
    fn test_size_fail() {
        let job1 = Job {
            code: 1,
            release_date: 1,
            processing_time: 4,
            due_date: 18,
            size: 10,
        };
        let job2 = Job {
            code: 2,
            release_date: 2,
            processing_time: 5,
            due_date: 19,
            size: 5,
        };
        let job3 = Job {
            code: 3,
            release_date: 3,
            processing_time: 6,
            due_date: 21,
            size: 4,
        };

        let job5 = Job {
            code: 5,
            release_date: 4,
            processing_time: 6,
            due_date: 25,
            size: 9,
        };

        let job6 = Job {
            code: 6,
            release_date: 4,
            processing_time: 7,
            due_date: 26,
            size: 9,
        };

        let job7 = Job {
            code: 7,
            release_date: 5,
            processing_time: 5,
            due_date: 28,
            size: 9,
        };

        let tester = Job {
            code: 4,
            release_date: 7,
            processing_time: 7,
            due_date: 18,
            size: 9
        };

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        batch1.insert(job1);
        batch1.insert(job2);
        batch1.insert(job3);
        batch2.insert(job5);
        batch2.insert(job6);
        batch3.insert(job7);

        let schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);

        let insertion_list = get_insertion_deviations(&schedule, &tester);
        let solution = vec![
            LogHistory {

            },
        ];
    }
}
