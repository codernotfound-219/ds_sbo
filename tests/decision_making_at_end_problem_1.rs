#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, BatchSchedule};
    use ds_sbo_rust::greedy_dp::{
        locate_eligible_batch, make_end_decision, EndDecision, InsertAction,
    };
    use ds_sbo_rust::resources::problem1::*;

    #[test]
    fn insertion_end_job10() {
        let job1 = job1();
        let job5 = job5();
        let job6 = job6();
        let tester = job10();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);

        batch1.insert(job5);
        batch1.insert(job6);
        batch2.insert(job1);

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);

        let actions: Vec<InsertAction> = vec![
           InsertAction::InsertInBatch {
                batch_index: 1,
                job_code: tester.code,
            },
        ];

        let decision = make_end_decision(&schedule, &tester);
        assert_eq!(decision, EndDecision::InsertAtLast(6, actions));
    }

    #[test]
    fn insertion_end_job2() {
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

        let decision = make_end_decision(&schedule, &tester);
        assert_eq!(decision, EndDecision::InsertAtLast(1, actions));
    }

    #[test]
    fn insertion_end_job3() {
        let job1 = job1();
        let job2 = job2();
        let job4 = job4();
        let job5 = job5();
        let job6 = job6();
        let job7 = job7();
        let job8 = job8();
        let job9 = job9();
        let job10 = job10();
        let tester = job3();

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
        batch5.insert(job2);
        batch5.insert(job10);

        let mut schedule = BatchSchedule::new();

        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);
        schedule.insert_end(batch5);

        if locate_eligible_batch(&schedule, tester.due_date).is_some() {
            panic!("Should have given no batch_index");
        }

        let decision = make_end_decision(&schedule, &tester);
        assert_eq!(decision, EndDecision::CreateAfter(5));
    }
}
