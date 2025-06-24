use super::utils::{calculate_deviation, size_check};
use crate::core::{BatchSchedule, Job};

pub fn insert_in(
    batch_index: usize,
    schedule: &BatchSchedule,
    job: &Job,
    prev_completion: u32,
    mut aggregate: i32,
) -> i32 {
    if size_check(&schedule.batches[batch_index], job) {
        let net_deviation = insert_in_size_ok(batch_index, schedule, job, prev_completion);
        if net_deviation < 0 {
            aggregate += net_deviation;
        }

        return if aggregate != 0 {
            aggregate
        } else {
            net_deviation
        };
    }

    let mut job_list: Vec<Job> = schedule.batches[batch_index].jobs.clone();

    if job_list.is_empty() {
        panic!("Tried to pop from an empty batch!");
    }
    let lp_job = job_list.pop().unwrap();

    let (current_deviation, completion) =
        compute_current_deviation(&job_list, job, prev_completion);
    if current_deviation < 0 {
        aggregate += current_deviation;
    }

    if batch_index + 1 == schedule.batches.len() {
        let release_date = lp_job.release_date.max(completion);
        let last_deviation =
            lp_job.due_date as i32 - (release_date as i32 + lp_job.processing_time as i32);
        if last_deviation < 0 {
            aggregate += last_deviation;
        }

        let net_deviation = current_deviation.min(last_deviation);

        return if aggregate != 0 {
            aggregate
        } else {
            net_deviation
        };
    }

    current_deviation.min(insert_in(
        batch_index + 1,
        schedule,
        &lp_job,
        completion,
        aggregate,
    ))
}

fn compute_current_deviation(list: &[Job], job: &Job, prev_completion: u32) -> (i32, u32) {
    let release_date = prev_completion
        .max(
            list.iter()
                .map(|cur_job| cur_job.release_date)
                .max()
                .unwrap(),
        )
        .max(job.release_date);
    let processing = job.processing_time.max(
        list.iter()
            .map(|cur_job| cur_job.processing_time)
            .max()
            .unwrap(),
    );
    let completion = processing + release_date;
    let due = job
        .due_date
        .min(list.iter().map(|cur_job| cur_job.due_date).min().unwrap());

    (due as i32 - completion as i32, completion)
}

fn insert_in_size_ok(
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

    calculate_deviation(batch_index+1, schedule, completion, deviation)
}
