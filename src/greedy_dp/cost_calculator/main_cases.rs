use crate::{core::{BatchSchedule, Job}, greedy_dp::Decision};
use super::{find_cost_inserting_in_batch, find_cost_creating_before, find_cost_creating_after, InsertAction};

pub fn make_decision(schedule: &BatchSchedule, batch_index: usize, job: &Job) -> Decision {
    let mut actions: Vec<InsertAction> = Vec::new();
    let cost_creating_before = find_cost_creating_before(schedule, batch_index, job);
    let cost_creating_after = find_cost_creating_after(schedule, batch_index, job);
    let cost_inserting_in = find_cost_inserting_in_batch(schedule, batch_index, job, i32::MAX, &mut actions);

    let options = [
        (cost_creating_before, Decision::CreateBefore(cost_creating_before)),
        (cost_creating_after, Decision::CreateAfter(cost_creating_after)),
        (cost_inserting_in, Decision::InsertAtPosition(cost_inserting_in)),
    ];

    let (_, decision) = options
        .iter()
        .max_by_key(|&(lateness, _)| lateness)
        .expect("At least one options is to be present");

    *decision
}
