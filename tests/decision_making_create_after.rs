#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, BatchSchedule};
    use ds_sbo_rust::greedy_dp::{find_cost_creating_after, locate_eligible_batch};
    use ds_sbo_rust::resources::problem2::*;

    #[test]
    fn check_value_6() {
        let job2 = job2();
        let job5 = job5();
        let job6 = job6();
        let tester = job1();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);

        batch1.insert(job2);
        batch2.insert(job6);
        batch3.insert(job5);

        let mut schedule = BatchSchedule::new();

        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);

        if let Some(batch_index) = locate_eligible_batch(&schedule, tester.due_date) {
            let cost = find_cost_creating_after(&schedule, batch_index, &tester);
            assert_eq!(batch_index, 1, "Failed index");
            assert_eq!(cost, 6, "Failed cost");
        } else {
            panic!("Should have found a batch");
        }
    }

    #[test]
    fn check_value_5() {
        let job2 = job2();
        let job5 = job5();
        let job6 = job6();
        let job1 = job1();
        let tester = job8();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);

        batch1.insert(job2);
        batch2.insert(job1);
        batch3.insert(job5);
        batch3.insert(job6);

        let mut schedule = BatchSchedule::new();

        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);

        if let Some(batch_index) = locate_eligible_batch(&schedule, tester.due_date) {
            let cost = find_cost_creating_after(&schedule, batch_index, &tester);
            assert_eq!(batch_index, 0, "Failed index");
            assert_eq!(cost, 5, "Failed cost");
        } else {
            panic!("Should have found a batch");
        }
    }

    #[test]
    fn check_value_4_v2() {
        let job2 = job2();
        let job5 = job5();
        let job6 = job6();
        let job1 = job1();
        let job8 = job8();
        let tester = job10();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);

        batch1.insert(job2);
        batch2.insert(job8);
        batch3.insert(job1);
        batch4.insert(job5);
        batch4.insert(job6);

        let mut schedule = BatchSchedule::new();

        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);

        if let Some(batch_index) = locate_eligible_batch(&schedule, tester.due_date) {
            let cost = find_cost_creating_after(&schedule, batch_index, &tester);
            assert_eq!(batch_index, 0, "Failed index");
            assert_eq!(cost, 4, "Failed cost");
        } else {
            panic!("Should have found a batch");
        }
    }
}
