use crate::structures::Job;

fn job1() -> Job {
    Job::new(1, 5, 28, 8, 8)
}
fn job2() -> Job {
    Job::new(2, 4, 21, 1, 7)
}
fn job3() -> Job {
    Job::new(3, 19, 19, 1, 8)
}
fn job4() -> Job {
    Job::new(4, 14, 39, 5, 6)
}
fn job5() -> Job {
    Job::new(5, 4, 41, 8, 8)
}
fn job6() -> Job {
    Job::new(6, 4, 39, 9, 9)
}
fn job7() -> Job {
    Job::new(7, 15, 42, 5, 9)
}
fn job8() -> Job {
    Job::new(8, 9, 21, 6, 6)
}
fn job9() -> Job {
    Job::new(9, 17, 39, 4, 9)
}
fn job10() -> Job {
    Job::new(10, 9, 19, 1, 7)
}

pub fn problem2() -> Vec<Job> {
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
