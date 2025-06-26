use crate::{core::{BatchSchedule, Job}, greedy_dp::{deviation_calculator::common::decisions, LogHistory}};
use super::utils::calculate_deviation;

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

pub fn get_creation_deviations(schedule: &BatchSchedule, job: &Job) -> Vec<LogHistory> {
    let mut result: Vec<LogHistory> = Vec::new();

    for index in 0 .. schedule.batches.len() {
        result.push(
            LogHistory::new(
                create_in(index, schedule, job),
                vec![decisions::create_at(index, job.code)],
            ),
        );
    }

    result.push(LogHistory::new(
        create_end(schedule, job),
        vec![decisions::create_end(job.code)],
    ));


    result
}
