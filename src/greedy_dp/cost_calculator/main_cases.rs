use crate::{core::{BatchSchedule, Job}, greedy_dp::Decision};
use super::{find_cost_creating_after, find_cost_creating_before, find_cost_inserting_after, find_cost_inserting_before, find_cost_inserting_in_batch, InsertAction};

pub fn make_decision(schedule: &BatchSchedule, batch_index: usize, job: &Job) -> Decision {
    let mut insert_in_actions: Vec<InsertAction> = Vec::new();
    let mut insert_before_actions: Vec<InsertAction> = Vec::new();
    let mut insert_after_actions: Vec<InsertAction> = Vec::new();
    let cost_inserting_before = find_cost_inserting_before(schedule, batch_index, job, &mut insert_before_actions);
    let cost_creating_before = find_cost_creating_before(schedule, batch_index, job);
    let cost_inserting_in = find_cost_inserting_in_batch(schedule, batch_index, job, i32::MAX, &mut insert_in_actions);
    let cost_creating_after = find_cost_creating_after(schedule, batch_index, job);
    let cost_inserting_after = find_cost_inserting_after(schedule, batch_index, job, &mut insert_after_actions);

    let options = [
        (cost_inserting_before, Decision::InsertBefore(cost_creating_before, insert_before_actions)),
        (cost_creating_before, Decision::CreateBefore(cost_creating_before)),
        (cost_inserting_in, Decision::InsertAtPosition(cost_inserting_in, insert_in_actions)),
        (cost_creating_after, Decision::CreateAfter(cost_creating_after)),
        (cost_inserting_after, Decision::InsertAfter(cost_inserting_in, insert_after_actions)),
    ];

    let (_, decision) = options
        .iter()
        .max_by_key(|&(lateness, _)| lateness)
        .expect("At least one options is to be present");

    decision.clone()
}
