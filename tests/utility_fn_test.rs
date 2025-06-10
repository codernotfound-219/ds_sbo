#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, BatchSchedule};
    use ds_sbo_rust::greedy_dp::utils::{create_before, create_after, insert_last, create_end, insert_at_position};
    use ds_sbo_rust::resources::problem2::*;

    #[test]
    fn test_create_before() {
        let job2 = job2();
        let job5 = job5();
        let job6 = job6();
        let tester = job1();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);

        batch1.insert_end(job2.clone());
        batch2.insert_end(job6.clone());
        batch2.insert_end(job5.clone());

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);

        create_before(&mut schedule, 1, tester.clone());

        let mut result = BatchSchedule::new();
        let mut result_batch1 = Batch::new(1);
        let mut result_batch2 = Batch::new(2);
        let mut result_batch3 = Batch::new(3);

        result_batch1.insert_end(job2);
        result_batch2.insert_end(tester);
        result_batch3.insert_end(job6);
        result_batch3.insert_end(job5);

        result.insert_end(result_batch1);
        result.insert_end(result_batch2);
        result.insert_end(result_batch3);

        assert_eq!(result, schedule);
    }

    #[test]
    fn test_create_after() {
        let job2 = job2();
        let job5 = job5();
        let job6 = job6();
        let job1 = job1();
        let tester = job8();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);

        batch1.insert_end(job2.clone());
        batch2.insert_end(job1.clone());
        batch3.insert_end(job6.clone());
        batch3.insert_end(job5.clone());

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);

        create_after(&mut schedule, 0, tester.clone());

        let mut result = BatchSchedule::new();
        let mut result_batch1 = Batch::new(1);
        let mut result_batch2 = Batch::new(2);
        let mut result_batch3 = Batch::new(3);
        let mut result_batch4 = Batch::new(4);

        result_batch1.insert_end(job2);
        result_batch2.insert_end(tester);
        result_batch3.insert_end(job1);
        result_batch4.insert_end(job6);
        result_batch4.insert_end(job5);

        result.insert_end(result_batch1);
        result.insert_end(result_batch2);
        result.insert_end(result_batch3);
        result.insert_end(result_batch4);

        assert_eq!(result, schedule);
    }

    #[test]
    fn test_create_end() {
        let job1 = job1();
        let job2 = job2();
        let job4 = job4();
        let job5 = job5();
        let job6 = job6();
        let tester = job7();
        let job8 = job8();
        let job10 = job10();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);
        let mut batch5 = Batch::new(5);

        batch1.insert_begin(job10.clone());
        batch1.insert_end(job2.clone());
        batch2.insert_begin(job8.clone());
        batch3.insert_begin(job1.clone());
        batch4.insert_begin(job4.clone());
        batch5.insert_end(job6.clone());
        batch5.insert_end(job5.clone());

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);
        schedule.insert_end(batch5);

        create_end(&mut schedule, tester.clone());

        let mut result = BatchSchedule::new();
        let mut result_batch1 = Batch::new(1);
        let mut result_batch2 = Batch::new(2);
        let mut result_batch3 = Batch::new(3);
        let mut result_batch4 = Batch::new(4);
        let mut result_batch5 = Batch::new(5);
        let mut result_batch6 = Batch::new(6);

        result_batch1.insert_begin(job10);
        result_batch1.insert_end(job2);
        result_batch2.insert_begin(job8);
        result_batch3.insert_begin(job1);
        result_batch4.insert_begin(job4);
        result_batch5.insert_end(job6);
        result_batch5.insert_end(job5);
        result_batch6.insert_end(tester);

        result.insert_end(result_batch1);
        result.insert_end(result_batch2);
        result.insert_end(result_batch3);
        result.insert_end(result_batch4);
        result.insert_end(result_batch5);
        result.insert_end(result_batch6);

        assert_eq!(result, schedule);
    }


}
