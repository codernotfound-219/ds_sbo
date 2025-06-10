use crate::core::{BatchSchedule, Job};

// NOTE:
// This function finds the minimum cost of creating a batch before
// the batch_index

pub fn find_cost_creating_before(schedule: &BatchSchedule, batch_index: usize, job: &Job) -> i32 {
    let batch = &schedule.batches[batch_index];
    let release_date = batch.release_date.max(job.release_date);
    let mut completion = release_date + job.processing_time;
    let cost_creating_before = job.due_date as i32 - completion as i32;

    let mut min_cost = i32::MAX;

    for index in batch_index .. schedule.batches.len() {
        let batch = &schedule.batches[index];
        let release_date = completion.max(batch.release_date);
        completion = release_date + batch.processing_time;

        let cost = batch.min_due_time as i32 - completion as i32;
        min_cost = min_cost.min(cost);
    }

    min_cost.min(cost_creating_before)
}

