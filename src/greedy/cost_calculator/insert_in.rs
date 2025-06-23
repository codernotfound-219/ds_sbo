use crate::core::{Job, BatchSchedule};
use crate::greedy::size_check;

// NOTE:
// This function calculates the cost of inserting at position:
// if size_check is ok:
//      -> iterates through batches, returns min_cost
// else:
//      -> recursively pops and moves jobs and keeps track of min_cost
//      -> returns the min_cost
//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InsertAction {
    InsertInBatch {batch_index: usize, job_code: u32},
    PopAndCreateNewBatch {batch_index: usize, job_code: u32},
    PopAndInsertInNextBatch {batch_index: usize, job_code: u32},
}

pub fn find_cost_inserting_in_batch(
    schedule: &BatchSchedule,
    batch_index: usize,
    cur_job: &Job,
    current_cost: i32,
    actions: &mut Vec<InsertAction>
) -> i32 {
    let batch = &schedule.batches[batch_index];

    if size_check(20, batch, cur_job) {
        actions.push(InsertAction::InsertInBatch { batch_index, job_code: cur_job.code });
        return current_cost.min(find_cost_inserting_size_ok(
            schedule,
            batch_index,
            cur_job,
        ));
    }

    let mut new_jobs = batch.jobs.clone();
    let last_job = new_jobs.pop().expect("No job to move");
    let completion_time = if batch_index == 0 {
        0
    } else {
        schedule.batches[batch_index-1].completion_time
    };

    // Cost of placing cur_job in cur_batch after popping the last_job of the batch.
    let (cost_of_current_batch, current_batch_completion) = compute_batch_cost_and_completion(&new_jobs, cur_job, completion_time);

    // Base case: if there are no more batches, create a new batch for the popped job
    if batch_index + 1 >= schedule.batches.len() {
        let cost_of_new_batch =
            last_job.due_date as i32 - (current_batch_completion as i32 + last_job.processing_time as i32);

        actions.push(InsertAction::InsertInBatch { batch_index, job_code: cur_job.code });
        actions.push(InsertAction::PopAndCreateNewBatch { batch_index, job_code: last_job.code });
        return current_cost
            .min(cost_of_current_batch)
            .min(cost_of_new_batch);
    }
    let cost_of_moving_last_job_as_new_batch = compute_remaining_batch_cost(schedule, &last_job, batch_index+1, current_batch_completion);
    // Recursive case: try to insert the last_job into the next batch
    let cost_of_inserting_in_next_batch = find_cost_inserting_in_batch(schedule, batch_index + 1, &last_job, current_cost.min(cost_of_current_batch), actions);

    let updated_current_cost = cost_of_moving_last_job_as_new_batch.min(cost_of_inserting_in_next_batch);

    actions.push(InsertAction::InsertInBatch { batch_index, job_code: cur_job.code });
    if updated_current_cost == cost_of_moving_last_job_as_new_batch {
        actions.push(InsertAction::PopAndCreateNewBatch { batch_index, job_code: last_job.code});
        cost_of_moving_last_job_as_new_batch
    } else {
        actions.push(InsertAction::PopAndInsertInNextBatch { batch_index, job_code: last_job.code});
        cost_of_inserting_in_next_batch
    }
}

fn compute_remaining_batch_cost(schedule: &BatchSchedule, popped_job: &Job, batch_index: usize, prev_completion: u32) -> i32 {
    let mut completion = prev_completion.max(popped_job.release_date) + popped_job.processing_time;
    let cost_of_new_batch = popped_job.due_date as i32- completion as i32;

    let mut min_cost = i32::MAX;

    for index in batch_index .. schedule.batches.len() {
        let batch = &schedule.batches[index];
        let release_date = completion.max(batch.release_date);
        completion = release_date + batch.processing_time;

        let cost = batch.min_due_time as i32 - completion as i32;
        min_cost = min_cost.min(cost);
    }

    cost_of_new_batch.min(min_cost)
}

fn find_cost_inserting_size_ok(schedule: &BatchSchedule, batch_index: usize, job: &Job) -> i32 {
    let batch = &schedule.batches[batch_index];

    let due_date = batch.min_due_time.min(job.due_date); // 19
    let release = batch.release_date.max(job.release_date); // 9
    let processing = batch.processing_time.max(job.processing_time); // 1
    let mut completion = release + processing; // 10
    let cost_inserting = due_date as i32 - completion as i32; // 19 - 10 = 9

    if batch_index + 1 == schedule.batches.len() {
        return cost_inserting;
    }

    let mut min_cost = i32::MAX;

    for index in batch_index+1 .. schedule.batches.len() {
        let batch = &schedule.batches[index];
        let release_date = completion.max(batch.release_date);
        completion = release_date + batch.processing_time;

        let cost = batch.min_due_time as i32 - completion as i32;
        min_cost = min_cost.min(cost);
    }

    cost_inserting.min(min_cost)
}

pub fn compute_batch_cost_and_completion(batch_list: &[Job], cur_job: &Job, release_date: u32) -> (i32, u32) {
    let mut max_release = cur_job.release_date.max(release_date);
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

    let completion = max_release + max_processing;
    ((min_due as i32 - completion as i32), completion)
}
