#[cfg(test)]
mod test {
    use ds_sbo_rust::core::{Batch, BatchSchedule, Decision};
    use ds_sbo_rust::algorithm2::find_cost_creating_before;
    use ds_sbo_rust::resources::problem2::*;

    #[test]
    fn test_value() {
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
    }
}
