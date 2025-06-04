#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, Job, Decision};
    use ds_sbo_rust::algorithm:: make_decision;

    // Shared test data as functions to create fresh instances

    #[test]
    fn test_insert_head() {
        let job1 = job1();
        let job6 = job6();
        let mut input = Batch::new(1);
        input.insert_begin(job1);

        let decision = make_decision(&mut input, &job6);
        assert_eq!(decision, Decision::InsertAtPosition(0));
    }

    #[test]
    fn test_move_job() {
        let job1 = job1();
        let job6 = job6();
        let mut input = Batch::new(1);
        input.insert_begin(job6);
        input.insert_end(job1);

        let decision = make_decision(&mut input, &job5());
        assert_eq!(decision, Decision::MoveLastJob(0));
    }

    #[test]
    fn test_next_batch() {
        let job5 = job5();
        let job6 = job6();

        let mut input = Batch::new(1);
        input.insert_begin(job5);
        input.insert_end(job6);

        let decision = make_decision(&mut input, &job10());
        assert_eq!(decision, Decision::CMPNextBatch);
    }

    #[test]
    fn test_create_batch() {
        let job5 = job5();
        let job6 = job6();

        let mut input = Batch::new(1);
        input.insert_begin(job5);
        input.insert_end(job6);

        let decision = make_decision(&mut input, &job7());
        assert_eq!(decision, Decision::CreateBatchAfter);
    }
}
