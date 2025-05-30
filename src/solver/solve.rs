use crate::structures::{BatchSchedule, Job};

pub fn solve(list: Vec<Job>) -> (BatchSchedule, u32) {
    let mut batch_schedule = BatchSchedule::new();
    let mut lateness: u32 = 0;

    (batch_schedule, lateness)
}
