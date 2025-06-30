use crate::structures::Job;

pub fn job1() -> Job {
    Job::new(1, 5, 24, 8, 8)
}
pub fn job2() -> Job {
    Job::new(2, 4, 21, 4, 7)
}
pub fn job3() -> Job {
    Job::new(3, 19, 25, 2, 8)
}
pub fn job4() -> Job {
    Job::new(4, 14, 28, 5, 6)
}
pub fn job5() -> Job {
    Job::new(5, 4, 22, 8, 8)
}
pub fn job6() -> Job {
    Job::new(6, 15, 34, 9, 9)
}
pub fn job7() -> Job {
    Job::new(7, 15, 28, 5, 9)
}
pub fn job8() -> Job {
    Job::new(8, 9, 21, 6, 6)
}
pub fn job9() -> Job {
    Job::new(9, 17, 34, 4, 9)
}
pub fn job10() -> Job {
    Job::new(10, 9, 19, 1, 7)
}
pub fn job11() -> Job {
    Job::new(11, 11, 22, 2, 5)
}
pub fn job12() -> Job {
    Job::new(12, 8, 24, 5, 8)
}
pub fn job13() -> Job {
    Job::new(13, 20, 39, 6, 6)
}
pub fn job14() -> Job {
    Job::new(14, 18, 38, 7, 5)
}
pub fn job15() -> Job {
    Job::new(15, 24, 40, 4, 6)
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
        job11(),
        job12(),
        job13(),
        job14(),
        job15(),
    ]
}
