use crate::core::{BatchSchedule, Job};
use crate::greedy_dp::{size_check, compute_batch_cost_and_completion, EndDecision};

use super::InsertAction;

// NOTE:
// This function handles the case when there are no eligible batches found:
// 1. create_new_batch at the end
// 2. Insert at the previous position
// It returns the decision with the maximum difference

pub fn make_end_decision(
    schedule: &BatchSchedule,
    job: &Job,
    actions: &mut Vec<InsertAction>,
) -> EndDecision {
    let last_batch = match schedule.batches.last() {
        Some(batch) => batch,
        None => panic!("Empty schedule was passed for decision-making"),
    };

    if size_check(20, last_batch, job) {
        let release_date = job.release_date.max(last_batch.release_date);
        let processing = job.processing_time.max(last_batch.processing_time);
        let completion = release_date + processing;
        let due = job.due_date.min(last_batch.min_due_time);

        let cost = due as i32 - completion as i32;
        actions.push(InsertAction::InsertInBatch {
            batch_index: schedule.batches.len() - 1,
            job_code: job.code,
        });
        return EndDecision::InsertAtLast(cost);
    }
    
    if last_batch.jobs.is_empty() {
        panic!("Tried to pop from an empty batch");
    }

    let mut job_list = last_batch.jobs.clone();
    let prev_completion_time = if last_batch.code == 1 {
        0
    } else {
        schedule.batches[last_batch.code - 2].completion_time
    };

    let last_job = job_list.pop().unwrap();
    let cost_of_creating_after = job.due_date as i32 - (last_batch.completion_time as i32 + job.processing_time as i32);

    if first_priority(&last_job, job) {
        return EndDecision::CreateAfter(cost_of_creating_after);
    }

    // This function computes the cost of placing the cur_job in the batch after
    // popping the last job
    let (cost_of_current_batch, completion_time) = compute_batch_cost_and_completion(&job_list, job, prev_completion_time);
    let cost_of_new_batch = last_job.due_date as i32 - (completion_time as i32 + last_job.processing_time as i32);
    actions.push(InsertAction::InsertInBatch { batch_index: schedule.batches.len() - 1, job_code: job.code });
    actions.push(InsertAction::PopAndCreateNewBatch { batch_index: schedule.batches.len() - 1, job_code: last_job.code });

    EndDecision::InsertAtLast(cost_of_new_batch.min(cost_of_current_batch))
}

// TODO: Subject to improvement
// - What if the both jobs have the same due date?
fn first_priority(current_job: &Job, other_job: &Job) -> bool {
    current_job.due_date <= other_job.due_date
}

// pub fn make_end_decision(schedule: &BatchSchedule, job: &Job) -> EndDecision {
//     let last_batch = &schedule.batches[schedule.batches.len() - 1];
//     let cost_of_creating_after =
//         job.due_date as i32 - (last_batch.completion_time as i32 + job.processing_time as i32);
//
//     if !(size_check(20, last_batch, job)) {
//         return EndDecision::CreateAfter(cost_of_creating_after);
//     }
//
//     let release_date = last_batch.release_date.max(job.release_date) as i32;
//     let processing_time = last_batch.processing_time.max(job.processing_time) as i32;
//     let cost_of_inserting_last = last_batch.min_due_time as i32 - (release_date + processing_time);
//
//     let options = [
//         (
//             cost_of_creating_after,
//             EndDecision::CreateAfter(cost_of_creating_after),
//         ),
//         (
//             cost_of_inserting_last,
//             EndDecision::InsertAtLast(cost_of_inserting_last),
//         ),
//     ];
//
//     let (_, decision) = options
//         .iter()
//         .max_by_key(|&(lateness, _)| lateness)
//         .expect("At least one options is to be present");
//
//     *decision
// }
