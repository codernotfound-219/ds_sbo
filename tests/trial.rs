#[cfg(test)]
mod test {
    use ds_sbo_rust::{algorithm::comparison, core::{Batch, BatchSchedule, Job}};

    #[test]
    fn scheduling() {
        let mut schedule = BatchSchedule::new();
        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        batch1.insert_end(job2());
        batch1.insert_end(job8());
        batch2.insert_end(job1());
        batch2.insert_end(job6());
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);

        panic!("{:#?}", schedule);
    }
    
    #[test]
    fn compare() {
        let job8 = job8();
        let job10 = job10();
        let decision = comparison(&job8, &job10, 0);

        panic!("{:#?}", decision);
    }
}

