use crate::core::{BatchSchedule, Job};

pub fn create_in(batch_index: usize, schedule: &BatchSchedule, job: &Job) -> i32 {
    let prev_completion = if batch_index == 0 {
        0
    } else {
        schedule.batches[batch_index - 1].completion_time
    };

    let mut release_date = job.release_date.max(prev_completion); // 14
    let mut completion = release_date + job.processing_time; // 17
    let mut deviation = job.due_date as i32 - completion as i32; // 5
    let mut aggregate = 0;

    for index in batch_index..schedule.batches.len() {
        let batch = &schedule.batches[index];
        release_date = batch.release_date.max(completion);
        completion = release_date + batch.processing_time;
        let current_deviation = batch.min_due_time as i32 - completion as i32;
        if current_deviation < 0 {
            aggregate += current_deviation;
        }
        deviation = deviation.min(current_deviation);
    }

    if aggregate != 0 {
        aggregate
    } else {
        deviation
    }
}

pub fn create_end(schedule: &BatchSchedule, job: &Job) -> i32 {
    let index = schedule.batches.len() - 1;
    let last_batch = &schedule.batches[index];
    let release = job.release_date.max(last_batch.completion_time);
    let completion = release + job.processing_time;

    job.due_date as i32 - completion as i32
}
