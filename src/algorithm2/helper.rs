use std::u32;

use super::EndDecision;
use crate::core::{Batch, BatchSchedule, Job};

// NOTE:
// This function locates the index of the first batch
// that has a due_date that is greater than the current-job's
// due_date.
// It also handles an edge case where you need to start from the batch
// that has the minimum min_due_time

pub fn locate_eligible_batch(schedule: &BatchSchedule, due: u32) -> Option<usize> {
    let min_index = schedule
        .batches
        .iter()
        .enumerate()
        .min_by_key(|(_, batch)| batch.min_due_time)
        .map(|(index, _)| index)?;

    schedule
        .batches
        .iter()
        .enumerate()
        .skip(min_index)
        .find(|(_, batch)| batch.min_due_time >= due)
        .map(|(index, _)| index)
}
// NOTE:
// This function handles the case when there are no eligible batches found:
// 1. create_new_batch at the end
// 2. Insert at the previous position
// It returns the decision with the maximum difference

pub fn make_end_decision(schedule: &BatchSchedule, job: &Job) -> EndDecision {
    let last_batch = &schedule.batches[schedule.batches.len() - 1];
    let cost_of_creating_after =
        job.due_date as i32 - (last_batch.completion_time as i32 + job.processing_time as i32);

    if !(size_check(20, last_batch, job)) {
        return EndDecision::CreateAfter(cost_of_creating_after);
    }

    let release_date = last_batch.release_date.max(job.release_date) as i32;
    let processing_time = last_batch.processing_time.max(job.processing_time) as i32;
    let cost_of_inserting_last = last_batch.min_due_time as i32 - (release_date + processing_time);

    let options = [
        (
            cost_of_creating_after,
            EndDecision::CreateAfter(cost_of_creating_after),
        ),
        (
            cost_of_inserting_last,
            EndDecision::InsertAtLast(cost_of_inserting_last),
        ),
    ];

    let (_, decision) = options
        .iter()
        .max_by_key(|&(lateness, _)| lateness)
        .expect("At least one options is to be present");

    *decision
}

// NOTE:
// This function finds the minimum cost of creating a batch before
// the batch_index

pub fn find_cost_creating_before(schedule: &BatchSchedule, batch_index: usize, job: &Job) -> i32 {
    let batch = &schedule.batches[batch_index];
    let release_date = batch.release_date.max(job.release_date);
    let cost_creating_before =
        job.due_date as i32 - (release_date as i32 + job.processing_time as i32);

    let num = batch.release_date as i32 - job.release_date as i32;
    let to_add = if num < 0 {
        job.processing_time + (-num) as u32
    } else {
        job.processing_time
    };

    let min_cost = schedule
        .batches
        .iter()
        .enumerate()
        .skip(batch_index)
        .map(|(_, batch)| {
            batch.min_due_time as i32 - (batch.completion_time as i32 + to_add as i32)
        })
        .min()
        .unwrap_or(i32::MAX);

    min_cost.min(cost_creating_before)
}

// NOTE:
// This function returns false if given job 
// cannot be placed in the given batch
pub fn size_check(capacity: u32, batch: &Batch, job: &Job) -> bool {
    batch.size + job.size <= capacity
}

pub fn find_cost_inserting_in_batch(
    schedule: &BatchSchedule,
    batch_index: usize,
    cur_job: &Job,
    current_cost: i32,
) -> i32 {
    let batch = &schedule.batches[batch_index];

    if size_check(20, batch, cur_job) {
        return current_cost.min(find_cost_inserting_size_ok(
            &schedule,
            batch_index,
            &cur_job,
        ));
    }

    let mut new_jobs = batch.jobs.clone();
    let last_job = new_jobs.pop().expect("No job to move");

    let cost_of_current_batch = compute_batch_cost(&new_jobs, &cur_job);
    let updated_current_cost = current_cost.min(cost_of_current_batch);

    // Base case: if there are no more batches, create a new batch for the popped job
    if batch_index + 1 >= schedule.batches.len() {
        let (max_release, max_processing, min_due) =
            new_jobs
                .iter()
                .fold((0, 0, u32::MAX), |(mut rel, mut proc, mut due), job| {
                    rel = rel.max(job.release_date);
                    proc = proc.max(job.processing_time);
                    due = due.min(job.due_date);
                    (rel, proc, due)
                });

        let completion = max_release + max_processing;

        let cost_of_current_batch = min_due as i32 - completion as i32;
        let cost_of_new_batch =
            last_job.due_date as i32 - (completion as i32 + last_job.processing_time as i32);

        return updated_current_cost
            .min(cost_of_current_batch)
            .min(cost_of_new_batch);
    }

    // Recursive case: try to insert the last_job into the next batch
    find_cost_inserting_in_batch(schedule, batch_index + 1, &last_job, updated_current_cost)
}

fn find_cost_inserting_size_ok(schedule: &BatchSchedule, batch_index: usize, job: &Job) -> i32 {
    let batch = &schedule.batches[batch_index];

    let due_date = batch.min_due_time.min(job.due_date);
    let release = batch.release_date.max(job.release_date);
    let processing = batch.processing_time.max(job.processing_time);
    let cost_inserting = due_date as i32 - (release as i32 + processing as i32);

    if batch_index + 1 == schedule.batches.len() {
        return cost_inserting;
    }

    let to_add = (release + processing) - batch.completion_time;

    let min_cost = schedule
        .batches
        .iter()
        .enumerate()
        .skip(batch_index + 1)
        .map(|(_, batch)| {
            batch.min_due_time as i32 - (batch.completion_time as i32 + to_add as i32)
        })
        .min()
        .unwrap_or(i32::MAX);

    return cost_inserting.min(min_cost);
}

fn compute_batch_cost(batch_list: &[Job], cur_job: &Job) -> i32 {
    let mut max_release = cur_job.release_date;
    let mut max_processing = cur_job.processing_time;
    let mut min_due = cur_job.due_date;

    for job in batch_list {
        if job.release_date > max_release {
            max_release = job.release_date;
        }
        if job.processing_time > max_processing {
            max_processing = job.processing_time;
        }
        if job.due_date < min_due {
            min_due = job.due_date;
        }
    }
    min_due as i32 - (max_release as i32 + max_processing as i32)
}
