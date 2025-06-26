use crate::structures::{BatchSchedule, Job};
use crate::greedy_dp::deviation_calculator::common::calculate_deviation;

// NOTE: this function returns the aggregated deviation due to insertion when size_check passes
pub fn calculate_deviation_for_direct_insertion(
    batch_index: usize,
    schedule: &BatchSchedule,
    job: &Job,
    prev_completion: u32,
) -> i32 {
    let batch = &schedule.batches[batch_index];
    let release_date = job
        .release_date
        .max(batch.release_date)
        .max(prev_completion);
    let processing = job.processing_time.max(batch.processing_time);
    let completion = release_date + processing;
    let due = job.due_date.min(batch.min_due_time);
    let deviation = due as i32 - completion as i32;

    calculate_deviation(batch_index + 1, schedule, completion, deviation)
}
