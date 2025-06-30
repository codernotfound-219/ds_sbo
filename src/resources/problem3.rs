use crate::structures::{Batch, BatchSchedule, Job};

pub fn job1() -> Job {
    Job::new(1, 12, 42, 1, 4)
}
pub fn job2() -> Job {
    Job::new(2, 16, 55, 10, 9)
}
pub fn job3() -> Job {
    Job::new(3, 10, 38, 7, 4)
}
pub fn job4() -> Job {
    Job::new(4, 19, 39, 6, 4)
}
pub fn job5() -> Job {
    Job::new(5, 6, 22, 1, 8)
}
pub fn job6() -> Job {
    Job::new(6, 3, 28, 2, 9)
}
pub fn job7() -> Job {
    Job::new(7, 5, 34, 10, 5)
}
pub fn job8() -> Job {
    Job::new(8, 12, 26, 7, 10)
}
pub fn job9() -> Job {
    Job::new(9, 18, 24, 5, 4)
}
pub fn job10() -> Job {
    Job::new(10, 17, 44, 2, 4)
}
pub fn job11() -> Job {
    Job::new(11, 2, 13, 5, 4)
}
pub fn job12() -> Job {
    Job::new(12, 6, 45, 9, 7)
}
pub fn job13() -> Job {
    Job::new(13, 14, 29, 8, 9)
}
pub fn job14() -> Job {
    Job::new(14, 12, 24, 4, 5)
}
pub fn job15() -> Job {
    Job::new(15, 7, 26, 9, 5)
}
pub fn job16() -> Job {
    Job::new(16, 1, 20, 2, 10)
}
pub fn job17() -> Job {
    Job::new(17, 14, 40, 2, 10)
}
pub fn job18() -> Job {
    Job::new(18, 7, 10, 1, 4)
}
pub fn job19() -> Job {
    Job::new(19, 2, 30, 1, 6)
}
pub fn job20() -> Job {
    Job::new(20, 11, 42, 2, 8)
}
pub fn job21() -> Job {
    Job::new(21, 2, 20, 5, 4)
}
pub fn job22() -> Job {
    Job::new(22, 3, 13, 7, 9)
}
pub fn job23() -> Job {
    Job::new(23, 8, 33, 7, 4)
}
pub fn job24() -> Job {
    Job::new(24, 2, 17, 2, 10)
}
pub fn job25() -> Job {
    Job::new(25, 12, 38, 9, 8)
}

pub fn problem3() -> Vec<Job> {
    vec![
        job1(),
        job2(),
        job3(),
        job4(),
        job5(),
        job6(),
        job7(),
        job8(),
        job9(),
        job10(),
        job11(),
        job12(),
        job13(),
        job14(),
        job15(),
        job16(),
        job17(),
        job18(),
        job19(),
        job20(),
        job21(),
        job22(),
        job23(),
        job24(),
        job25(),
    ]
}

pub fn solution() -> BatchSchedule {
    let mut batch1 = Batch::new(1);
    let mut batch2 = Batch::new(2);
    let mut batch3 = Batch::new(3);
    let mut batch4 = Batch::new(4);
    let mut batch5 = Batch::new(5);

    batch1.insert(job11());
    batch1.insert(job6());
    batch2.insert(job5());
    batch2.insert(job8());
    batch2.insert(job14());
    batch3.insert(job15());
    batch3.insert(job13());
    batch3.insert(job9());
    batch4.insert(job7());
    batch4.insert(job3());
    batch4.insert(job4());
    batch4.insert(job1());
    batch5.insert(job10());
    batch5.insert(job12());
    batch5.insert(job2());

    let mut schedule = BatchSchedule::new();
    schedule.insert_end(batch1);
    schedule.insert_end(batch2);
    schedule.insert_end(batch3);
    schedule.insert_end(batch4);
    schedule.insert_end(batch5);

    schedule
}
