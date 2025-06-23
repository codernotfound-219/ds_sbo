#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, BatchSchedule};
    use ds_sbo_rust::greedy_dp::{InsertAction, locate_eligible_batch, make_decision, Decision};
    use ds_sbo_rust::resources::problem1::*;

    #[test]
    fn insertion_in_position_of_job9() {
        let job1 = job1();
        let job4 = job4();
        let job5 = job5();
        let job6 = job6();
        let job7 = job7();
        let job8 = job8();
        let job10 = job10();
        let tester = job2();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);

        batch1.insert(job5);
        batch1.insert(job6);
        batch2.insert(job7);
        batch3.insert(job8);
        batch3.insert(job4);
        batch4.insert(job1);
        batch4.insert(job10);

        let mut schedule = BatchSchedule::new();

        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);

        if locate_eligible_batch(&schedule, tester.due_date).is_some() {
            panic!("Should have given no batch_index");
        }

        let actions: Vec<InsertAction> = vec![
            InsertAction::InsertInBatch {
                batch_index: 3,
                job_code: tester.code,
            },
            InsertAction::PopAndCreateNewBatch {
                batch_index: 3,
                job_code: 10,
            },
        ];
        //
        // let decision = make_end_decision(&schedule, &tester);
        // assert_eq!(decision, EndDecision::InsertAtLast(1, actions));
    }
}
