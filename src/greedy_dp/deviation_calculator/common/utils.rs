use crate::structures::{Batch, Job, BatchSchedule};
use crate::resources::BATCH_CAPACITY;
use super::types::InsertionResult;

// NOTE: returns true if cur_job can be inserted in given batch
pub fn size_check(batch: &Batch, job: &Job) -> bool {
    batch.size + job.size <= BATCH_CAPACITY
}

// NOTE: Calculate the deviation from a given batch index onwards
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

// NOTE:
// this function calculates the deviation due to insertion of cur_job after displacing lp_job
pub fn compute_current_deviation(jobs: &[Job], cur_job: &Job, prev_completion: u32) -> InsertionResult {
    let release_date = prev_completion
        .max(
            jobs.iter()
                .map(|cur_job| cur_job.release_date)
                .max()
                .unwrap_or(0),
        )
        .max(cur_job.release_date);
    
    let processing = cur_job.processing_time.max(
        jobs.iter()
            .map(|cur_job| cur_job.processing_time)
            .max()
            .unwrap_or(0),
    );
    
    let completion = processing + release_date;
    let due = cur_job.due_date.min(
        jobs.iter()
            .map(|cur_job| cur_job.due_date)
            .min()
            .unwrap_or(u32::MAX)
    );

    InsertionResult {
        deviation: due as i32 - completion as i32,
        completion,
    }
}
