#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, BatchSchedule};
    use ds_sbo_rust::resources::problem2::*;

    #[test]
    fn insert_begin() {
        let job2 = job2();
        let job5 = job5();
        let job6 = job6();
        let tester = job1();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut new_batch = Batch::new(1);

        batch1.insert_begin(job2);
        batch2.insert_begin(job5);
        batch2.insert_begin(job6);
        new_batch.insert_begin(tester);

        let mut result = BatchSchedule::new();
        result.insert_end(batch1.clone());
        result.insert_end(new_batch.clone());
        result.insert_end(batch2.clone());

        let mut input = BatchSchedule::new();
        input.insert_begin(batch2);
        input.insert_begin(batch1);

        input.insert_at_position(1, new_batch);

        assert_eq!(result, input);
    }
    
}
