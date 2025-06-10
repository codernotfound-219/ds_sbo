use crate::core::{BatchSchedule, Job};
use crate::algorithm2::EndDecision;
use crate::algorithm2::helper::size_check;

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

