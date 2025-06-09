#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, BatchSchedule};
    use ds_sbo_rust::algorithm2::{find_cost_creating_before, locate_eligible_batch};
    use ds_sbo_rust::resources::problem2::*;

    #[test]
    fn test_value_19() {
        let job2 = job2();
        let job5 = job5();
        let tester = job6();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        batch1.insert_end(job2);
        batch2.insert_end(job5);

        let mut schedule = BatchSchedule::new();
        schedule.insert_begin(batch1);
        schedule.insert_end(batch2);

        if let Some(batch_index) = locate_eligible_batch(&schedule, tester.due_date) {
            let cost = find_cost_creating_before(&schedule, batch_index, &tester);
            assert_eq!(batch_index, 1);
            assert_eq!(cost, 19);
        } else {
            panic!("Should have found an eligible batch");
        }
    }

    #[test]
    fn find_min_at_first() {
        let job2 = job2();
        let job5 = job5();
        let job6 = job6();
        let tester = job1();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);

        batch1.insert_begin(job2);
        batch2.insert_begin(job5);
        batch2.insert_begin(job6);

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);

        if let Some(batch_index) = locate_eligible_batch(&schedule, tester.due_date) {
            let cost = find_cost_creating_before(&schedule, batch_index, &tester);
            assert_eq!(batch_index, 1);
            assert_eq!(cost, 15);
        } else {
            panic!("Should have found an eligible batch");
        }
    }

    #[test]
    fn test_value_1() {
        let job1 = job1();
        let job2 = job2();
        let job5 = job5();
        let job6 = job6();
        let job8 = job8();
        let job10 = job10();
        let tester = job4();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);

        batch1.insert_begin(job2);
        batch1.insert_begin(job10);
        batch2.insert_begin(job8);
        batch3.insert_begin(job1);
        batch4.insert_begin(job5);
        batch4.insert_begin(job6);

        let mut schedule = BatchSchedule::new();
        schedule.insert_begin(batch4);
        schedule.insert_begin(batch3);
        schedule.insert_begin(batch2);
        schedule.insert_begin(batch1);

        if let Some(batch_index) = locate_eligible_batch(&schedule, tester.due_date) {
            let cost = find_cost_creating_before(&schedule, batch_index, &tester);
            assert_eq!(batch_index, 3);
            assert_eq!(cost, 1);
        } else {
            panic!("Should have found an eligible batch");
        }
    }
}
