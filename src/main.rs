use ds_sbo_rust::structures::Job;

fn main() {
    let job1 = Job::new(1, 1, 29, 9, 9);
    let job2 = Job::new(2, 16, 31, 4, 7);
    let job3 = Job::new(3, 20, 42, 5, 9);
    let job4 = Job::new(4, 14, 22, 3, 6);
    let job5 = Job::new(5, 5, 22, 6, 6);
    let job6 = Job::new(6, 4, 27, 9, 9);
    let job7 = Job::new(7, 13, 17, 1, 8);
    let job8 = Job::new(8, 13, 22, 4, 9);
    let job9 = Job::new(9, 18, 28, 8, 6);
    let job10 = Job::new(10, 6, 38, 4, 7);

    let list = vec![job1, job2, job3, job4, job5, job6, job7, job8, job9, job10];

    for job in &list {
        std::println!("{}", job);
    }

    // let mut batch_schedule = BatchSchedule::new();
    //
    // batch_schedule.add_batch(Batch::new(1, vec![job5, job6]));
    // batch_schedule.add_batch(Batch::new(2, vec![job7]));
    // batch_schedule.add_batch(Batch::new(3, vec![job8, job4]));
    // batch_schedule.add_batch(Batch::new(4, vec![job9, job1]));
    // batch_schedule.add_batch(Batch::new(5, vec![job2, job10]));
    // batch_schedule.add_batch(Batch::new(6, vec![job3]));
    //
    // for batch in batch_schedule.get_batches() {
    //     std::println!("{}", batch);
    // }
}
