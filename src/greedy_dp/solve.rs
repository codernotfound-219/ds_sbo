use super::{EndDecision, Decision};
use crate::core::{Job, Batch, BatchSchedule};
use super::{locate_eligible_batch, make_decision, make_end_decision};
use super::utils::{create_end, create_after, create_before, insert_last, insert_at_position};

pub fn solve(list: &mut Vec<Job>) -> BatchSchedule {
    let mut schedule = BatchSchedule::new();
    let mut batch = Batch::new(1);

    Job::sort_release_date(list);
    batch.insert(list.pop().unwrap());
    schedule.insert_begin(batch);

    loop {
        if list.is_empty() {
            break;
        }
        let current = list.pop().expect("ERROR: expected a job in the sorted list");

        if let Some(batch_index) = locate_eligible_batch(&schedule, current.due_date) {
                let result = make_decision(&schedule, batch_index, &current);

                match result {
                    Decision::CreateAfter(_) => {create_after(&mut schedule, batch_index, current);},
                    Decision::CreateBefore(_) => {create_before(&mut schedule, batch_index, current);},
                    Decision::InsertAtPosition(_, actions) => {insert_at_position(&mut schedule, current, &actions);},
                    Decision::InsertBefore(_, actions) => {insert_at_position(&mut schedule, current, &actions);},
                    Decision::InsertAfter(_, actions) => {insert_at_position(&mut schedule, current, &actions);},
                }
            } else {
                // let result = make_end_decision(&schedule, &current);
                //
                // match result {
                //     EndDecision::CreateAfter(_) => {create_end(&mut schedule, current);},
                //     EndDecision::InsertAtLast(_) => {insert_last(&mut schedule, current);},
                // }
            }
    }

    schedule
}
