use crate::core::{Batch, BatchSchedule, Job};
use super::{comparison, handlers::Decision, size_check, Priority, Status};

pub fn solve(list: &mut Vec<Job>) -> (BatchSchedule, u32) {
    let mut batch_schedule = BatchSchedule::new();
    let lateness: u32 = 0;
    let mut index: u32 = 1;

    Job::sort_release_date(list);

    while !list.is_empty() {
        if batch_schedule.is_empty() {
            let mut batch = Batch::new(index);
            batch.insert_begin(list.pop().unwrap());
            batch_schedule.insert_begin(batch);
        }

        index += 1;
    }

    (batch_schedule, lateness)
}

pub fn make_decision(batch: &mut Batch, cur: Job) -> Decision {
    for (index, head) in batch.jobs.iter().enumerate() {
        let result = comparison(&head, &cur);

        match (result.priority, result.status) {
            (Priority::Head, Status::Pass) => continue,
            (Priority::Head, Status::Fail(_))=> return Decision::CMPNextBatch,
            (Priority::Current, Status::Pass) => {
                if size_check(20, batch, &cur) {
                    return Decision::InsertAtPosition(index);
                } else {
                    return Decision::MoveLastJob;
                }
            }, 
            (Priority::Current, Status::Fail(_))=> return Decision::CreateNewBatch,
        }
    }

    Decision::CMPNextBatch
}
