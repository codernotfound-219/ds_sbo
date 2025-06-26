use super::calculations::{create_end, create_in};
use crate::{
    core::{BatchSchedule, Job},
    greedy_dp::{deviation_calculator::common::decisions, LogHistory},
};

pub fn get_creation_deviations(schedule: &BatchSchedule, job: &Job) -> Vec<LogHistory> {
    let mut result: Vec<LogHistory> = Vec::new();

    for index in 0..schedule.batches.len() {
        result.push(LogHistory::new(
            create_in(index, schedule, job),
            vec![decisions::create_at(index, job.code)],
        ));
    }

    result.push(LogHistory::new(
        create_end(schedule, job),
        vec![decisions::create_end(job.code)],
    ));

    result
}
