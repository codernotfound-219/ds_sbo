use crate::core::{BatchSchedule, Job};
use super::super::common::calculate_deviation;

// NOTE:
// this function calculates the deviation due to create at the given index
pub fn create_in(batch_index: usize, schedule: &BatchSchedule, job: &Job) -> i32 {
    let prev_completion = if batch_index == 0 {
        0
    } else {
        schedule.batches[batch_index - 1].completion_time
    };

    let release_date = job.release_date.max(prev_completion);
    let completion = release_date + job.processing_time;
    let deviation = job.due_date as i32 - completion as i32;

    calculate_deviation(batch_index, schedule, completion, deviation)
}

// NOTE:
// this function calculates the deviation due to creation at the end
pub fn create_end(schedule: &BatchSchedule, job: &Job) -> i32 {
    let index = schedule.batches.len() - 1;
    let last_batch = &schedule.batches[index];
    let release = job.release_date.max(last_batch.completion_time);
    let completion = release + job.processing_time;

    job.due_date as i32 - completion as i32
}

