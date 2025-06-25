#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{BatchSchedule, Batch};
    use ds_sbo_rust::greedy_dp::deviation_calculator::{create_in, create_end};
    use ds_sbo_rust::resources::problem1::*;

    #[test]
    fn test_create_in_for_j4() {
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

        let deviation_at_0 = create_in(0, &schedule, &tester);
        let deviation_at_1 = create_in(1, &schedule, &tester);
        let deviation_at_2 = create_in(2, &schedule, &tester);
        let deviation_at_3 = create_in(3, &schedule, &tester);

        assert_eq!(deviation_at_0, -34, "failed deviation_0");
        assert_eq!(deviation_at_1, -3, "failed deviation_1");
        assert_eq!(deviation_at_2, -2, "failed deviation_2");
        assert_eq!(deviation_at_3, -2, "failed deviation_3");
    }

    #[test]
    fn test_create_end() {
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

        let deviation_j4 = create_end(&schedule, &job4);
        assert_eq!(deviation_j4, -9, "failed deviation_j4");

        schedule.batches[2].insert(job4);

        let deviation_j2 = create_end(&schedule, &job2);
        assert_eq!(deviation_j2, -1, "failed deviation_j2");
    }

}
