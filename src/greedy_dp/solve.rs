use std::error::Error;
use std::time::Instant;
use crate::structures::{Batch, BatchSchedule, Job};
use crate::tardiness_calculator::get_tardiness;

use super::helper::solver_helper;

pub fn solve(list: &mut Vec<Job>) -> Result<BatchSchedule, Box<dyn Error>> {
    if list.is_empty() {
        return Err("Empty list of jobs passed to solver".into());
    }

    let start = Instant::now();

    let mut schedule = BatchSchedule::new();
    let mut batch = Batch::new(1);

    Job::sort_due_date(list);
    batch.insert(list.pop().unwrap());
    schedule.insert_begin(batch);

    loop {
        if list.is_empty() { break; }
        solver_helper(&mut schedule, list.pop().unwrap());
    }

    let tardiness3 = get_tardiness(&schedule);
    let duration = start.elapsed().as_nanos();

    println!();
    println!("Solving using Greedy-DP: ");
    println!();
    println!("------------------------------------");
    println!("total tardiness: {}", tardiness3);
    println!("computation time: {} ns", duration);
    println!("------------------------------------");
    println!();

    Ok(schedule)
}
