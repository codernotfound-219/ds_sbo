use crate::structures::{Batch, BatchSchedule, Job};

fn job1() -> Job {
    Job::new(1, 1, 29, 9, 9)
}
fn job2() -> Job {
    Job::new(2, 16, 31, 4, 7)
}
fn job3() -> Job {
    Job::new(3, 20, 42, 5, 9)
}
fn job4() -> Job {
    Job::new(4, 14, 22, 3, 6)
}
fn job5() -> Job {
    Job::new(5, 5, 22, 6, 6)
}
fn job6() -> Job {
    Job::new(6, 4, 27, 9, 9)
}
fn job7() -> Job {
    Job::new(7, 13, 17, 1, 8)
}
fn job8() -> Job {
    Job::new(8, 13, 22, 4, 9)
}
fn job9() -> Job {
    Job::new(9, 18, 28, 8, 6)
}
fn job10() -> Job {
    Job::new(10, 6, 38, 4, 7)
}

pub fn problem1() -> Vec<Job> {
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
    ]
}
