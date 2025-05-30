use ds_sbo_rust::batch::Batch;
use ds_sbo_rust::job::Job;
use ds_sbo_rust::batch_schedule::BatchSchedule;

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

    let mut order = BatchSchedule::new();

    order.add_batch(Batch::new(1, vec![job5, job6]));
    order.add_batch(Batch::new(2, vec![job7]));
    order.add_batch(Batch::new(3, vec![job8, job4]));
    order.add_batch(Batch::new(4, vec![job9, job1]));
    order.add_batch(Batch::new(5, vec![job2, job10]));
    order.add_batch(Batch::new(6, vec![job3]));

    for batch in order.get_batches() {
        std::println!("{}", batch);
    }
}
