use crate::core::{BatchSchedule, Job};

// NOTE:
// this function calculates the deviation as defined the report, from the given batch index
fn calculate_deviation(
    batch_index: usize,
    schedule: &BatchSchedule,
    mut completion: u32,
    mut deviation: i32,
) -> i32 {
    let mut aggregate = if deviation < 0 { deviation } else { 0 };

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

    if aggregate != 0 {
        aggregate
    } else {
        deviation
    }
}

fn insert_in(batch_index: usize, schedule: &BatchSchedule, job: &Job) -> i32 {
    let batch = &schedule.batches[batch_index];
    let release_date = job.release_date.max(batch.release_date);
    let processing = job.processing_time.max(batch.processing_time);
    let completion = release_date + processing;
    let due = job.due_date.min(batch.min_due_time);
    let deviation = due as i32 - completion as i32;

    calculate_deviation(batch_index, schedule, completion, deviation)
}

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
