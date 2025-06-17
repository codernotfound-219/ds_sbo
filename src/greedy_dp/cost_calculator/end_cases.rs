use super::InsertAction;
use crate::core::{BatchSchedule, Job};
use crate::greedy_dp::{size_check, compute_batch_cost_and_completion, EndDecision};

// NOTE:
// This function handles the case when there are no eligible batches found:
// 1. create_new_batch at the end
// 2. Insert at the previous position
// It returns the decision with the maximum difference

pub fn make_end_decision(schedule: &BatchSchedule, job: &Job) -> EndDecision {
    let mut insert_end_actions: Vec<InsertAction> = Vec::new();
    let cost_of_inserting_last = find_cost_inserting_at_last(schedule, job, &mut insert_end_actions);

    if cost_of_inserting_last >= 0 {
        return EndDecision::InsertAtLast(cost_of_inserting_last, insert_end_actions);
    }

    let cost_of_creating_after = find_cost_creating_after_end(schedule, job);

    let options = [
        (
            cost_of_inserting_last,
            EndDecision::InsertAtLast(cost_of_inserting_last, insert_end_actions),
        ),
        (
            cost_of_creating_after,
            EndDecision::CreateAfter(cost_of_creating_after),
        ),
    ];

    let (_, decision) = options
        .iter()
        .max_by_key(|&(lateness, _)| lateness)
        .expect("At least one options is to be present");

    decision.clone()
}

fn find_cost_inserting_at_last(schedule: &BatchSchedule, cur_job: &Job, actions: &mut Vec<InsertAction>) -> i32 {
    let last_batch = match schedule.batches.last() {
        Some(batch) => batch,
        None => panic!("Empty schedule was passed for decision-making"),
    };

    if size_check(20, last_batch, cur_job) {
        let release_date = cur_job.release_date.max(last_batch.release_date);
        let processing = cur_job.processing_time.max(last_batch.processing_time);
        let completion = release_date + processing;
        let due = cur_job.due_date.min(last_batch.min_due_time);

        let cost = due as i32 - completion as i32;

        actions.push(InsertAction::InsertInBatch {
            batch_index: last_batch.code - 1,
            job_code: cur_job.code,
        });

        return cost;
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
    if first_priority(&last_job, cur_job) {
        return i32::MIN;
    }

    let (cost_of_current_batch, completion_time) = compute_batch_cost_and_completion(&job_list, cur_job, prev_completion_time);
    let cost_of_new_batch = last_job.due_date as i32 - (completion_time as i32 + last_job.processing_time as i32);

    actions.push(InsertAction::InsertInBatch {
        batch_index: last_batch.code-1,
        job_code: cur_job.code,
    });
    actions.push(InsertAction::PopAndCreateNewBatch {
        batch_index: last_batch.code-1,
        job_code: last_job.code,
    });
    cost_of_current_batch.min(cost_of_new_batch)
}

fn find_cost_creating_after_end(schedule: &BatchSchedule, cur_job: &Job) -> i32 {
    let last_batch = match schedule.batches.last() {
        Some(batch) => batch,
        None => panic!("Empty Schedule was given for decision-making"),
    };

    let release_date = last_batch.completion_time.max(cur_job.release_date);
    let completion = release_date + cur_job.processing_time;
    let cost = cur_job.due_date as i32 - completion as i32;

    cost
}

// TODO: Subject to improvement
// - What if the both jobs have the same due date?
fn first_priority(current_job: &Job, other_job: &Job) -> bool {
    current_job.due_date <= other_job.due_date
}
