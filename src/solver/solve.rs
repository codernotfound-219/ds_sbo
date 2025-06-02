use crate::structures::{Batch, BatchSchedule, Job};

#[derive(Debug, PartialEq)]
enum Priority {
    Head,
    Current,
}

#[derive(Debug, PartialEq)]
enum Status {
    Pass,      // Both jobs can meet their deadlines
    Fail(u32), // At least one job will be late (value = max lateness)
}

#[derive(Debug)]
struct CMP {
    priority: Priority,
    status: Status,
}

pub fn solve(list: &mut Vec<&Job>) -> (BatchSchedule, u32) {
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

fn size_check(size: u32, batch: &Batch, job: &Job) -> bool {
    batch.size + job.size <= size
}

fn comparison(head: &Job, current: &Job) -> CMP {
    let batch_release = head.release_date.max(current.release_date);
    let batch_processing = head.processing_time.max(current.processing_time);
    let batch_completion = batch_release + batch_processing;

    let head_slack = head.due_date as i32 - batch_completion as i32;
    let current_slack = current.due_date as i32 - batch_completion as i32;

    let priority = determine_priority(head_slack, current_slack);
    let status = calculate_status(head_slack, current_slack);

    CMP { priority, status }
}

fn determine_priority(head_slack: i32, current_slack: i32) -> Priority {
    match head_slack.cmp(&current_slack) {
        std::cmp::Ordering::Less => Priority::Head,
        std::cmp::Ordering::Greater => Priority::Current,
        std::cmp::Ordering::Equal => Priority::Head,
    }
}

fn calculate_status(head_slack: i32, current_slack: i32) -> Status {
    let head_lateness = if head_slack < 0 {
        (-head_slack) as u32
    } else {
        0
    };
    let current_lateness = if current_slack < 0 {
        (-current_slack) as u32
    } else {
        0
    };

    match (head_lateness, current_lateness) {
        (0, 0) => Status::Pass,
        (head_late, current_late) => Status::Fail(head_late.max(current_late)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparison_basic() {
        let job1 = Job::new(1, 10, 20, 5, 3); // Due: 20, Processing: 5
        let job2 = Job::new(2, 12, 25, 4, 2); // Due: 25, Processing: 4

        let result = comparison(&job1, &job2);

        // job1 should have priority due to earlier deadline
        assert_eq!(result.priority, Priority::Head);
    }

    #[test]
    fn test_comparison_deadline_violation() {
        let job1 = Job::new(1, 10, 15, 8, 3); // Will be late
        let job2 = Job::new(2, 12, 25, 4, 2); // On time

        let result = comparison(&job1, &job2);

        // Should indicate failure due to job1 being late
        match result.status {
            Status::Fail(lateness) => assert!(lateness > 0),
            Status::Pass => panic!("Expected failure status"),
        }
    }

    #[test]
    fn test_size_check() {
        let job = Job::new(1, 10, 20, 5, 3);
        let mut batch = Batch::new(1);
        batch.insert_begin(&Job::new(2, 5, 15, 3, 4));

        assert!(size_check(10, &batch, &job)); // 4 + 3 <= 10
        assert!(!size_check(6, &batch, &job)); // 4 + 3 > 6
    }

    // #[test]
    // fn final_check() {
    //     let job1 = Job::new(1, 1, 29, 9, 9);
    //     let job2 = Job::new(2, 16, 31, 4, 7);
    //     let job3 = Job::new(3, 20, 42, 5, 9);
    //     let job4 = Job::new(4, 14, 22, 3, 6);
    //     let job5 = Job::new(5, 5, 22, 6, 6);
    //     let job6 = Job::new(6, 4, 27, 9, 9);
    //     let job7 = Job::new(7, 13, 17, 1, 8);
    //     let job8 = Job::new(8, 13, 22, 4, 9);
    //     let job9 = Job::new(9, 18, 28, 8, 6);
    //     let job10 = Job::new(10, 6, 38, 4, 7);
    //
    //     let mut list = vec![&job1, &job2, &job3, &job4, &job5, &job6, &job7, &job8, &job9, &job10];
    //
    //     let result = solve(&mut list);
    //     let lateness = 1;
    //
    //     let mut answer = BatchSchedule::new();
    //     let mut batch1 = Batch::new(1);
    //     let mut batch2 = Batch::new(2);
    //     let mut batch3 = Batch::new(3);
    //     let mut batch4 = Batch::new(4);
    //     let mut batch5 = Batch::new(5);
    //     let mut batch6 = Batch::new(6);
    //
    //     batch1.insert_begin(&job5);
    //     batch1.insert_end(&job6);
    //     batch2.insert_begin(&job7);
    //     batch3.insert_begin(&job8);
    //     batch3.insert_end(&job4);
    //     batch4.insert_begin(&job9);
    //     batch4.insert_end(&job1);
    //     batch5.insert_begin(&job2);
    //     batch5.insert_end(&job10);
    //     batch6.insert_begin(&job3);
    //
    //     answer.insert_end(batch6);
    //     answer.insert_end(batch5);
    //     answer.insert_end(batch4);
    //     answer.insert_end(batch3);
    //     answer.insert_end(batch2);
    //     answer.insert_end(batch1);
    //
    // }
}
