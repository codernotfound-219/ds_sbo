use crate::core::{Batch, BatchSchedule, Job};
use super::{find_cost_inserting_in_batch, InsertAction};

// NOTE:
// This function finds the minimum cost of inserting in the batch before
// the batch_index

pub fn find_cost_inserting_before(
    schedule: &BatchSchedule,
    batch_index: usize,
    job: &Job,
    actions: &mut Vec<InsertAction>,
) -> i32 {
    if batch_index == 0 {
        return i32::MIN;
    }

    let prev_batch = &schedule.batches[batch_index - 1];
    if priority_is_current(prev_batch, job) {
        return find_cost_inserting_in_batch(schedule, batch_index - 1, job, i32::MAX, actions);
    }

    i32::MIN
}

fn priority_is_current(batch: &Batch, job: &Job) -> bool {
    let mut job_list = batch.jobs.clone();

    if let Some(last_job) = job_list.pop() {
        last_job.due_date < job.due_date
    } else {
        panic!("Tried to pop from empty batch in 'find_cost_inserting_before'");
    }
}
