use crate::structures::BatchSchedule;
use super::structure::Lateness;

pub fn get_tardiness(schedule: &BatchSchedule) -> i32 {
    let mut aggregate = 0;
    let late_list = helper(schedule);

    for each_lateness in late_list {
        aggregate += each_lateness.lateness;
        println!(
            "J{} is late by: {}",
            each_lateness.job_code, each_lateness.lateness
        );
    }

    aggregate
}

fn helper(schedule: &BatchSchedule) -> Vec<Lateness> {
    let mut late_job_list: Vec<Lateness> = Vec::new();
    for batch in schedule.batches.iter() {
        for job in batch.jobs.iter() {
            let completion = job.due_date as i32 - batch.completion_time as i32;

            if completion < 0 {
                late_job_list.push(Lateness::new(completion.abs(), job.code));
            }
        }
    }

    late_job_list
}
