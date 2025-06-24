use crate::core::{Batch, Job, BatchSchedule};
use crate::resources::BATCH_CAPACITY;

// NOTE:
// this function returns true if job can be placed in batch
pub fn size_check(batch: &Batch, job: &Job) -> bool {
    batch.size + job.size <= BATCH_CAPACITY
}

// NOTE:
// this function calculates the deviation as defined the report, from the given batch index
pub fn calculate_deviation(
    batch_index: usize,
    schedule: &BatchSchedule,
    mut completion: u32,
    mut deviation: i32,
) -> i32 {
    let mut aggregate = if deviation < 0 { deviation } else { 0 };

    // if batch_index >= schedule.batches.len() the loop is not executed
    for index in batch_index..schedule.batches.len() {
        let batch = &schedule.batches[index];
        let release_date = batch.release_date.max(completion);
        completion = release_date + batch.processing_time;
        let current_deviation = batch.min_due_time as i32 - completion as i32;

        if current_deviation < 0 {
            aggregate += current_deviation
        }

        deviation = deviation.min(current_deviation);
    }

    if aggregate != 0 { aggregate } else { deviation }
}

