use ds_sbo_rust::core::Batch;

fn print_batch_details(batch: &mut Batch) -> &mut Batch {
    println!("{:#?}", batch);
    batch
}

#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, BatchSchedule, Decision};
    use ds_sbo_rust::algorithm::make_decision;
    use ds_sbo_rust::resources::problem1;

    use crate::print_batch_details;


    #[test]
    fn insert_at_position() {
        let job6 = problem1::job6();
        let job1 = problem1::job1();
        let tester = problem1::job5();

        let mut batch1 = Batch::new(1);
        batch1.insert_end(job6);
        batch1.insert_end(job1);

        let mut schedule = BatchSchedule::new();
        schedule.insert_begin(batch1);

        let (decision, popped_jobs) = make_decision(&mut schedule, &tester);
        assert_eq!(decision, Decision::InsertAtPosition(0, 0), "make_decision() gave the wrong Decision");
        
        let popped = popped_jobs.expect("make_decision() did not pop the job");
        assert_eq!(popped.len(), 1, "Popped one too many or too little jobs");
        assert_eq!(popped[0].code, 1, "Popped the wrong job");
    }

    #[test]
    fn create_batch_after() {
        let job1 = problem1::job1();
        let job5 = problem1::job5();
        let job6 = problem1::job6();
        let job10 = problem1::job10();
        let tester = problem1::job7();

        let mut batch1 = Batch::new(1);
        batch1.insert_end(job5);
        batch1.insert_end(job6);

        let mut batch2 = Batch::new(2);
        batch2.insert_begin(job1);
        batch2.insert_begin(job10);

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);

        let (decision, popped_jobs) = make_decision(&mut schedule, &tester);
        assert_eq!(decision, Decision::CreateBatchAfter(0));
        assert!(popped_jobs.is_none());
    }
}
