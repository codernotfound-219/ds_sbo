use crate::algorithm::{Priority, Status, CMP};
use crate::core::{Batch, Job};

pub fn size_check(size: u32, batch: &Batch, job: &Job) -> bool {
    batch.size + job.size <= size
}

pub fn comparison(head: &Job, current: &Job, prev_completion: u32) -> CMP {
    let batch_release = head
        .release_date
        .max(current.release_date)
        .max(prev_completion);
    let batch_processing = head.processing_time.max(current.processing_time);
    let batch_completion = batch_release + batch_processing;

    let head_slack = head.due_date as i32 - batch_completion as i32;
    let current_slack = current.due_date as i32 - batch_completion as i32;

    let priority = determine_priority(head.due_date, current.due_date);
    let status = calculate_status(head_slack, current_slack);

    CMP { priority, status }
}

pub fn determine_priority(head_slack: u32, current_slack: u32) -> Priority {
    match head_slack.cmp(&current_slack) {
        std::cmp::Ordering::Less => Priority::Head,
        std::cmp::Ordering::Greater => Priority::Current,
        std::cmp::Ordering::Equal => Priority::Head,
    }
}

pub fn calculate_status(head_slack: i32, current_slack: i32) -> Status {
    let minimum = head_slack.min(current_slack);

    if minimum >= 0 {
        return Status::Pass;
    }

    Status::Fail(minimum)
}
