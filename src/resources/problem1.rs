use crate::structures::{Batch, BatchSchedule, Job};

pub fn job1() -> Job {
    Job::new(1, 1, 29, 9, 9)
}
pub fn job2() -> Job {
    Job::new(2, 16, 31, 4, 7)
}
pub fn job3() -> Job {
    Job::new(3, 20, 42, 5, 9)
}
pub fn job4() -> Job {
    Job::new(4, 14, 22, 3, 6)
}
pub fn job5() -> Job {
    Job::new(5, 5, 22, 6, 6)
}
pub fn job6() -> Job {
    Job::new(6, 4, 27, 9, 9)
}
pub fn job7() -> Job {
    Job::new(7, 13, 17, 1, 8)
}
pub fn job8() -> Job {
    Job::new(8, 13, 22, 4, 9)
}
pub fn job9() -> Job {
    Job::new(9, 18, 28, 8, 6)
}
pub fn job10() -> Job {
    Job::new(10, 6, 38, 4, 7)
}

// Returns the solution of greed_dp on problem 1
pub fn greed_dp_solution() -> BatchSchedule {
    let mut batch1 = Batch::new(1);
    let mut batch2 = Batch::new(2);
    let mut batch3 = Batch::new(3);
    let mut batch4 = Batch::new(4);
    let mut batch5 = Batch::new(5);
    let mut batch6 = Batch::new(6);
    let mut schedule = BatchSchedule::new();

    batch1.insert(job5());
    batch1.insert(job6());
    batch2.insert(job7());
    batch3.insert(job8());
    batch3.insert(job4());
    batch4.insert(job9());
    batch4.insert(job1());
    batch5.insert(job2());
    batch5.insert(job10());
    batch6.insert(job3());

    schedule.insert_end(batch1);
    schedule.insert_end(batch2);
    schedule.insert_end(batch3);
    schedule.insert_end(batch4);
    schedule.insert_end(batch5);
    schedule.insert_end(batch6);

    schedule
}
