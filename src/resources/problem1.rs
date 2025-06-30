use crate::structures::Job;

pub fn job1() -> Job {
    Job::new(1, 1, 29, 9, 9, Some(2))
}
pub fn job2() -> Job {
    Job::new(2, 16, 31, 4, 7, Some(6))
}
pub fn job3() -> Job {
    Job::new(3, 20, 42, 5, 9, Some(2))
}
pub fn job4() -> Job {
    Job::new(4, 14, 22, 3, 6, Some(10))
}
pub fn job5() -> Job {
    Job::new(5, 5, 22, 6, 6, Some(3))
}
pub fn job6() -> Job {
    Job::new(6, 4, 27, 9, 9, Some(6))
}
pub fn job7() -> Job {
    Job::new(7, 13, 17, 1, 8, Some(9))
}
pub fn job8() -> Job {
    Job::new(8, 13, 22, 4, 9, Some(3))
}
pub fn job9() -> Job {
    Job::new(9, 18, 28, 8, 6, Some(3))
}
pub fn job10() -> Job {
    Job::new(10, 6, 38, 4, 7, Some(5))
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
