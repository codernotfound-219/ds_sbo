#[cfg(test)]
mod test {
    use ds_sbo_rust::algorithm2::{locate_eligible_batch, Decision, create_or_insert_last};
    use ds_sbo_rust::core::{Batch, BatchSchedule};
    use ds_sbo_rust::resources::problem2::*;

    #[test]
    fn check_none_and_create_after_1() {
        let job1 = job1();
        let job2 = job2();
        let job4 = job4();
        let job5 = job5();
        let job6 = job6();
        let job8 = job8();
        let job10 = job10();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);
        let mut batch5 = Batch::new(5);

        batch1.insert_begin(job10);
        batch1.insert_begin(job2);
        batch2.insert_begin(job8);
        batch3.insert_begin(job1);
        batch4.insert_begin(job4);
        batch5.insert_begin(job6);
        batch5.insert_begin(job5);

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);
        schedule.insert_end(batch5);

        let tester = job7();
        let result_index = locate_eligible_batch(&schedule, tester.due_date);
        assert_eq!(result_index, None);

        let decision = create_or_insert_last(&schedule, &tester);
        assert_eq!(decision, Decision::CreateAfter(-1));
    }

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

    #[test]
    fn check_none_and_create_after_2() {
        let job2 = job2();
        let tester = job5();

        let mut batch1 = Batch::new(1);
        batch1.insert_begin(job2);

        let mut schedule = BatchSchedule::new();
        schedule.insert_begin(batch1);

        let result_index = locate_eligible_batch(&schedule, tester.due_date);
        assert_eq!(result_index, None);

        let decision = create_or_insert_last(&schedule, &tester);
        assert_eq!(decision, Decision::CreateAfter(28));
    }
}
