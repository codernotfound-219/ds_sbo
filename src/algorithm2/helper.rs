use crate::core::{Batch, BatchSchedule, Job};

// NOTE:
// This function locates the index of the first batch
// that has a due_date that is greater than the current-job's
// due_date.
// It also handles an edge case where you need to start from the batch
// that has the minimum min_due_time

pub fn locate_eligible_batch(schedule: &BatchSchedule, due: u32) -> Option<usize> {
    let min_index = schedule
        .batches
        .iter()
        .enumerate()
        .min_by_key(|(_, batch)| batch.min_due_time)
        .map(|(index, _)| index)?;

    schedule
        .batches
        .iter()
        .enumerate()
        .skip(min_index)
        .find(|(_, batch)| batch.min_due_time >= due)
        .map(|(index, _)| index)
}
// NOTE:
// This function returns false if given job 
// cannot be placed in the given batch
pub fn size_check(capacity: u32, batch: &Batch, job: &Job) -> bool {
    batch.size + job.size <= capacity
}
