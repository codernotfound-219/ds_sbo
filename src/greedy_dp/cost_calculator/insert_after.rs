use crate::core::{BatchSchedule, Job};
use super::{find_cost_inserting_in_batch, InsertAction};

// NOTE:
// This function finds the minimum cost of inserting in the batch after
// the batch_index

pub fn find_cost_inserting_after(
    schedule: &BatchSchedule,
    batch_index: usize,
    job: &Job,
    actions: &mut Vec<InsertAction>,
) -> i32 {
    if batch_index-1 == schedule.batches.len() {
        return i32::MIN;
    }
    find_cost_inserting_in_batch(schedule, batch_index+1, job, i32::MAX, actions)
}

