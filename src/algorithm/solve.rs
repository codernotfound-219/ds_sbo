use std::u32::MAX;

use crate::core::{Batch, BatchSchedule, Decision, Job};
use super::{comparison, size_check, Priority, Status};

pub fn solve(list: &mut Vec<Job>) -> (BatchSchedule, u32) {
    let mut batch_schedule = BatchSchedule::new();
    let lateness: u32 = 0;
    let mut index: usize = 1;

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

// NOTE: This function makes a decision:
//  1. CreateBatchAfter
//  2. CreateBatchBefore
//  3. InsertAtPosition
// by comparing the current job from the sorted list, with the given batch.

pub fn make_decision(schedule: &mut BatchSchedule, cur: &Job) -> (Decision, Option<Vec<Job>>) {
    for (batch_index, batch) in schedule.batches.iter_mut().enumerate() {
        if (batch.completion_time <= cur.release_date) { continue; }

        for (job_index, head) in batch.jobs.iter().enumerate() {
            let result = comparison(&head, &cur, batch.completion_time);

            match (result.priority, result.status) {
                (Priority::Current, Status::Pass) => {
                    if size_check(20, &batch, cur) {
                        return (Decision::InsertAtPosition(batch_index, job_index), None);
                    } else {
                        // TODO:
                        // 1. check if removing last job frees up space
                        // 2. see if last job can be placed in the next batch
                        let pop_job = batch.pop_job().unwrap();
                },
                (Priority::Head, Status::Pass) => {
                    if job_index+1 == batch.jobs.len() {
                        break;
                    }
                    continue;
                },
                (Priority::Head, Status::Fail(lateness_at_position)) => {
                    let mut lateness_creation_before = head.due_date - (batch.completion_time + cur.processing_time);
                    let mut lateness_creation_after = cur.due_date - (batch.completion_time + cur.processing_time);
                    lateness_creation_before = lateness_creation_before.min(get_lateness(batch_index, schedule, cur));
                    lateness_creation_after = lateness_creation_after.min(get_lateness(batch_index+1, schedule, cur));

                    let options = [
                        (lateness_at_position, Decision::InsertAtPosition(batch_index, job_index+1)),
                        (lateness_creation_before, Decision::CreateBatchBefore),
                        (lateness_creation_after, Decision::CreateBatchAfter),
                    ];

                    let (_, decision) = options
                        .iter()
                        .min_by_key(|&(lateness, _)| lateness)
                        .expect("At least one options is to be present");

                    return (*decision, None);
                },
                (Priority::Current, Status::Fail(lateness_at_position)) => {
                    let mut lateness_creation_before = cur.due_date - (batch.release_date + cur.processing_time);
                    let mut lateness_creation_after = cur.due_date - (batch.completion_time + cur.processing_time);
                    lateness_creation_before = lateness_creation_before.min(get_lateness(batch_index, schedule, cur));
                    lateness_creation_after = lateness_creation_after.min(get_lateness(batch_index+1, schedule, cur));

                    let options = [
                        (lateness_at_position, Decision::InsertAtPosition(batch_index, job_index)),
                        (lateness_creation_before, Decision::CreateBatchBefore),
                        (lateness_creation_after, Decision::CreateBatchAfter),
                    ];

                    let (_, decision) = options
                        .iter()
                        .min_by_key(|&(lateness, _)| lateness)
                        .expect("At least one options is to be present");

                    return (*decision, None);
                },
            }
        }
    }


    (Decision::CreateBatchBefore, None) // BUG: remove this
}

fn get_lateness(batch_index: usize, schedule: &BatchSchedule, job: &Job) -> u32 {
    schedule.batches
    .get(batch_index..)
    .and_then(|batches| {
        batches.iter()
            .map(|batch| {
                batch.min_due_time - (batch.completion_time + job.processing_time)  
            })
            .min()
        })
        .unwrap()
}

// pub fn make_decision(batch: &mut Batch, cur: &Job) -> (Decision, Option<Vec<Job>>) {
//     for (index, head) in batch.jobs.iter().enumerate() {
//         let result = comparison(&head, &cur, batch.completion_time);

//         match (result.priority, result.status) {
//             (Priority::Current, Status::Pass) => {
//                 if size_check(20, batch, &cur) {
//                     return (Decision::InsertAtPosition(index), None);
//                 } else {
//                     let mut pop_jobs = Vec::new();

//                     while batch.size > 20 {
//                         if let Some(job) = batch.pop_job() {
//                             pop_jobs.push(job);
//                         }
//                     }

//                     return (Decision::InsertAtPosition(index), Some(pop_jobs));
//                 }
//             }, 
//             (Priority::Head, Status::Pass) => {
//                 continue;
//             },
//             (Priority::Head, Status::Fail(lateness_at_position))=> {
//                 // NOTE: compare next_batch, if next_batch is null, create_new_batch

//                 let lateness_creation_before = head.due_date - (batch.completion_time + cur.processing_time);
//                 let lateness_creation_after = cur.due_date - (batch.completion_time + cur.processing_time);

//                 let options = [
//                     (lateness_at_position, Decision::InsertAtPosition(index+1)),
//                     (lateness_creation_before, Decision::CreateBatchBefore),
//                     (lateness_creation_after, Decision::CreateBatchAfter),
//                 ];

//                 let (_, decision) = options
//                     .iter()
//                     .min_by_key(|&(lateness, _)| lateness)
//                     .expect("At least one options is to be present");

//                 return (decision.clone(), None);
//             },
//             (Priority::Current, Status::Fail(lateness_at_position))=> {
//                 //NOTE: check creation before, insert at position, creation after
//                 // choose: the choice with minimum lateness.

//                 let lateness_creation_before = cur.due_date - (batch.release_date + cur.processing_time);
//                 let lateness_creation_after = cur.due_date - (batch.completion_time + cur.processing_time);

//                 let options = [
//                     (lateness_at_position, Decision::InsertAtPosition(index)),
//                     (lateness_creation_after, Decision::CreateBatchAfter),
//                     (lateness_creation_before, Decision::CreateBatchBefore),
//                 ];
                
//                 let (_, decision) = options
//                     .iter()
//                     .min_by_key(|&(lateness, _)| lateness)
//                     .expect("At least one option to be present");

//                 return (decision.clone(), None);
//             },
//         }
//     }

//     (Decision::CMPNextBatch, None)
// }

// pub fn execute(schedule: &mut BatchSchedule, job: Job) {
//     let batch_len = schedule.batches.len();
//
//     for batch_index in 0..batch_len {
//         let mut cur_batch = schedule.batches[batch_index];
//         let decision = make_decision(&mut cur_batch, &job);
//
//         match decision {
//             Decision::MoveLastJob(pos) => {
//                 let pop_job = cur_batch.jobs.last().unwrap();
//                 if cur_batch.size - pop_job.size <= 20 {
//                     let next_batch_possibility = make_decision(&mut schedule.batches[batch_index+1], &job);
//                 }
//             }
//             Decision::CMPNextBatch => {
//                 if batch_index == batch_len-1 {
//                     let mut batch = Batch::new(batch_len + 1);
//                     batch.insert_begin(job);
//                     schedule.insert_end(batch);
//                     break;
//                 } else { continue; }
//             },
//             Decision::InsertAtPosition(pos) => {
//                 cur_batch.insert_at_position(pos, job);
//                 break;
//             },
//             Decision::CreateBatchBefore =>
//         }
//     }
// }

// fn move_job(batch_index: usize, batch_size: usize, schedule: &mut BatchSchedule, job: Job) -> Decision {
//     if batch_index == batch_size {
//         if let Some(current) = can_create_next(&schedule.batches[batch_index-1], &job) {
//             return current;
//         } else {
//             return Decision::CreateBatchAfter;
//         }
//     }
//
//     let next_batch = schedule.batches[batch_index+1];
//     let decision = make_decision(&mut next_batch, &job);
//
//     match decision {
//
//     }
// }

// fn can_create_next(batch: &Batch, job: &Job) -> Option<Decision> {
//     if job.due_date >= job.processing_time + batch.completion_time {
//         return None;
//     }
//
//     Some(batch.log.unwrap().past_decision)
// }
