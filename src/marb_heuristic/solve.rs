use std::error::Error;
use std::time::Instant;

use crate::{
    structures::{Job, Batch, BatchSchedule},
    tardiness_calculator::get_tardiness,
};
use super::{
    structure::MarbBatch,
    helper::{compute_min_due_date, find_job},
    execute::get_formed_batches,
};

pub fn solve(list: &mut Vec<Job>) -> Result<BatchSchedule, Box<dyn Error>> {
    let start = Instant::now();

    let mut schedule: BatchSchedule = BatchSchedule::new();
    let mut batch_indexer = 1;

    while !list.is_empty() {
        let formed_batches: Vec<MarbBatch> = get_formed_batches(list.clone())?;
        let priority_batch = compute_min_due_date(&formed_batches);
        
        let mut batch = Batch::new(batch_indexer);
        batch_indexer += 1;
        for job in formed_batches[priority_batch].jobs.iter() {
            if let Some(list_index) = find_job(list, job) {
                list.remove(list_index);
            } else {
                return Err("Could not find the job from batch in main list!".into());
            }
            batch.insert(*job);
        }
        schedule.insert_end(batch);
    }

    let tardiness3 = get_tardiness(&schedule);
    let duration = start.elapsed().as_nanos();

    println!();
    println!("Solving using MARB Heuristic: ");
    println!();
    println!("------------------------------------");
    println!("total tardiness: {}", tardiness3);
    println!("computation time: {} ns", duration);
    println!("------------------------------------");
    println!();
    Ok(schedule)
}


#[cfg(test)]
mod test {
    use crate::marb_heuristic::execute::get_formed_batches;
    use crate::marb_heuristic::structure::MarbBatch;
    use crate::resources::problem3::*;

    #[test]
    fn final_test() {
        let result = get_formed_batches(problem3()).ok().unwrap();
        let job1 = job1();
        let job2 = job2();
        let job3 = job3();
        let job4 = job4();
        let job5 = job5();
        let job6 = job6();
        let job7 = job7();
        let job8 = job8();
        let job9 = job9();
        let job10 = job10();
        let job11 = job11();
        let job12 = job12();
        let job13 = job13();
        let job14 = job14();
        let job15 = job15();
        let job16 = job16();
        let job17 = job17();
        let job18 = job18();
        let job19 = job19();
        let job20 = job20();
        let job21 = job21();
        let job22 = job22();
        let job23 = job23();
        let job24 = job24();
        let job25 = job25();

        let mut batch1 = MarbBatch::new(1);
        let mut batch2 = MarbBatch::new(2);
        let mut batch3 = MarbBatch::new(3);
        let mut batch4 = MarbBatch::new(4);
        let mut batch5 = MarbBatch::new(5);
        let mut batch6 = MarbBatch::new(6);
        let mut batch7 = MarbBatch::new(7);
        let mut batch8 = MarbBatch::new(8);
        let mut batch9 = MarbBatch::new(9);

        batch1.insert(job18);
        batch1.insert(job5);
        batch1.insert(job19);
        batch2.insert(job11);
        batch2.insert(job22);
        batch2.insert(job21);
        batch3.insert(job24);
        batch3.insert(job16);
        batch4.insert(job9);
        batch4.insert(job14);
        batch4.insert(job8);
        batch5.insert(job15);
        batch5.insert(job6);
        batch5.insert(job23);
        batch6.insert(job13);
        batch6.insert(job7);
        batch6.insert(job3);
        batch7.insert(job25);
        batch7.insert(job4);
        batch7.insert(job20);
        batch8.insert(job17);
        batch8.insert(job1);
        batch8.insert(job10);
        batch9.insert(job12);
        batch9.insert(job2);

        let formed_batches = vec![
            batch1, batch2, batch3, batch4, batch5, batch6, batch7, batch8, batch9,
        ];

        assert_eq!(result, formed_batches);
    }
}
