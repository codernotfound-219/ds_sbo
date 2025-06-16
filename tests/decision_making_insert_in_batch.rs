#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, BatchSchedule};
    use ds_sbo_rust::greedy_dp::cost_calculator::InsertAction;
    use ds_sbo_rust::greedy_dp::{locate_eligible_batch, find_cost_inserting_in_batch};
    use ds_sbo_rust::resources::problem2::*;

    #[test]
    fn check_value_0() {
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
        let mut actions: Vec<InsertAction> = Vec::new();

        if let Some(batch_index) = locate_eligible_batch(&schedule, tester.due_date) {
            let cost = find_cost_inserting_in_batch(&schedule, batch_index, &tester, i32::MAX, &mut actions);
            println!("{:#?}", actions);
            assert_eq!(batch_index, 3, "Failed index");
            assert_eq!(cost, 0, "Failed cost");
        } else {
            panic!("Should have gotten an eligible batch");
        }
    }

    #[test]
    fn test_value_5() {
        let job2 = job2();
        let job1 = job1();
        let job5 = job5();
        let job6 = job6();
        let tester = job8();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);

        batch1.insert_begin(job2);
        batch2.insert_begin(job1);
        batch3.insert_begin(job5);
        batch3.insert_begin(job6);

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        let mut actions: Vec<InsertAction> = Vec::new();

        if let Some(batch_index) = locate_eligible_batch(&schedule, tester.due_date) {
            let cost = find_cost_inserting_in_batch(&schedule, batch_index, &tester, i32::MAX, &mut actions);
            println!("{:#?}", actions);
            assert_eq!(batch_index, 0);
            assert_eq!(cost, 5);
        } else {
            panic!("Should have found batch");
        }
    }

    #[test]
    fn check_value_14() {
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
        let mut actions: Vec<InsertAction> = Vec::new();

        if let Some(batch_index) = locate_eligible_batch(&schedule, tester.due_date) {
            let cost = find_cost_inserting_in_batch(&schedule, batch_index, &tester, i32::MAX, &mut actions);
            println!("{:#?}", actions);
            assert_eq!(batch_index, 1);
            assert_eq!(cost, 14);
        } else {
            panic!("Should have found batch");
        }
    }

    #[test]
    fn check_value_4() {
        let job1 = job1();
        let job2 = job2();
        let job5 = job5();
        let job6 = job6();
        let job8 = job8();
        let tester = job10();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);

        batch1.insert_begin(job2);
        batch2.insert_begin(job8);
        batch3.insert_begin(job1);
        batch4.insert_begin(job5);
        batch4.insert_begin(job6);

        let mut schedule = BatchSchedule::new();
        schedule.insert_begin(batch4);
        schedule.insert_begin(batch3);
        schedule.insert_begin(batch2);
        schedule.insert_begin(batch1);
        let mut actions: Vec<InsertAction> = Vec::new();

        if let Some(batch_index) = locate_eligible_batch(&schedule, tester.due_date) {
            let cost = find_cost_inserting_in_batch(&schedule, batch_index, &tester, i32::MAX, &mut actions);
            println!("{:#?}", actions);
            assert_eq!(batch_index, 0, "Failed index");
            assert_eq!(cost, 4, "Failed cost");
        } else {
            panic!("Should have gotten an eligible batch");
        }
    }

}
