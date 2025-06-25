use crate::core::{Batch, BatchSchedule, Job};

pub fn solve(list: &mut Vec<Job>) -> BatchSchedule {
    if list.is_empty() {
        panic!("Empty SLUJ passed to solver");
    }

    let mut schedule = BatchSchedule::new();
    let mut batch = Batch::new(1);

    Job::sort_release_date(list);
    batch.insert(list.pop().unwrap());
    schedule.insert_begin(batch);

    schedule
}
