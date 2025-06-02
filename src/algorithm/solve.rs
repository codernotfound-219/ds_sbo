use crate::core::{Batch, BatchSchedule, Job};

pub fn solve(list: &mut Vec<Job>) -> (BatchSchedule, u32) {
    let mut batch_schedule = BatchSchedule::new();
    let lateness: u32 = 0;
    let mut index: u32 = 1;

    Job::sort_release_date(list);

    while !list.is_empty() {
        if batch_schedule.is_empty() {
            let mut batch = Batch::new(index);
            batch.insert_begin(list.pop().unwrap());
            batch_schedule.insert_begin(batch);
        }

        index += 1;
    }

    (batch_schedule, lateness)
}


