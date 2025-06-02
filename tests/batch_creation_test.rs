#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, Job};
    use ds_sbo_rust::algorithm::{Decision, make_decision};

    // Shared test data as functions to create fresh instances
    fn job1() -> Job {
        Job::new(1, 1, 29, 9, 9)
    }
    fn job2() -> Job {
        Job::new(2, 16, 31, 4, 7)
    }
    fn job3() -> Job {
        Job::new(3, 20, 42, 5, 9)
    }
    fn job4() -> Job {
        Job::new(4, 14, 22, 3, 6)
    }
    fn job5() -> Job {
        Job::new(5, 5, 22, 6, 6)
    }
    fn job6() -> Job {
        Job::new(6, 4, 27, 9, 9)
    }
    fn job7() -> Job {
        Job::new(7, 13, 17, 1, 8)
    }
    fn job8() -> Job {
        Job::new(8, 13, 22, 4, 9)
    }
    fn job9() -> Job {
        Job::new(9, 18, 28, 8, 6)
    }
    fn job10() -> Job {
        Job::new(10, 6, 38, 4, 7)
    }


    #[test]
    fn test_insert_head() {
        let job1 = job1();
        let job6 = job6();
        let mut input = Batch::new(1);
        input.insert_begin(job1);

        let decision = make_decision(&mut input, job6);
        assert_eq!(decision, Decision::InsertAtPosition(0));
    }

    #[test]
    fn test_move_job() {
        let job1 = job1();
        let job6 = job6();
        let mut input = Batch::new(1);
        input.insert_begin(job6);
        input.insert_end(job1);

        let decision = make_decision(&mut input, job5());
        assert_eq!(decision, Decision::MoveLastJob);
    }

    #[test]
    fn test_next_batch() {
        let job5 = job5();
        let job6 = job6();

        let mut input = Batch::new(1);
        input.insert_begin(job5);
        input.insert_end(job6);

        let decision = make_decision(&mut input, job10());
        assert_eq!(decision, Decision::CMPNextBatch);
    }

    #[test]
    fn test_create_batch() {
        let job5 = job5();
        let job6 = job6();

        let mut input = Batch::new(1);
        input.insert_begin(job5);
        input.insert_end(job6);

        let decision = make_decision(&mut input, job7());
        assert_eq!(decision, Decision::CreateNewBatch);
    }
}
