#[cfg(test)]
mod test {
    use ds_sbo_rust::greedy_dp::deviation_calculator::insertion;
    use ds_sbo_rust::greedy_dp::{get_action, get_creation_deviations, get_insertion_deviations};
    use ds_sbo_rust::resources::problem3::*;
    use ds_sbo_rust::structures::{BatchSchedule, Batch};
    use ds_sbo_rust::tardiness_calculator::get_tardiness;

    #[test]
    fn check_prob3_solution() {
        let job11= job11();
        let job6= job6();
        let job7= job7();
        let job12 = job12();
        let job5= job5();
        let job15 = job15();
        let job3 = job3();
        let job14 = job14();
        let job8 = job8();
        let job1 = job1();
        let job13 = job13();
        let job2 = job2();
        let job10 = job10();
        let job9 = job9();
        let job4 = job4();
        
        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);
        let mut batch5 = Batch::new(5);
        // let mut batch6 = Batch::new(6);

        batch1.insert(job11);
        batch1.insert(job6);
        batch2.insert(job14);
        batch2.insert(job5);
        batch2.insert(job8);
        batch3.insert(job15);
        batch3.insert(job9);
        batch3.insert(job13);
        batch4.insert(job7);
        batch4.insert(job3);
        batch4.insert(job1);
        batch4.insert(job4);
        batch5.insert(job10);
        batch5.insert(job12);
        batch5.insert(job2);
        // batch3.insert(job8);
        // batch3.insert(job14);
        // batch3.insert(job9);
        // batch4.insert(job13);
        // batch4.insert(job3);
        // batch4.insert(job1);
        // batch5.insert(job10);
        // batch5.insert(job12);
        // batch5.insert(job4);

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);
        schedule.insert_end(batch5);
        // schedule.insert_end(batch6);


        println!("{}", schedule);
        assert_eq!(true, false);
    }

    #[test]
    fn test_tardiness() {
        let job1 = job1();
        let job2 = job2();
        let job3 = job3();
        let job4 = job4();
        let job5 = job5();
        let job6 = job6();
        let job7 = job7();
        let job8 = job8();
        let job9 = job9();
        let job10 = job10();
        let job11 = job11();
        let job12 = job12();
        let job13 = job13();
        let job14 = job14();
        let job15 = job15();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);
        let mut batch5 = Batch::new(5);
        let mut batch6 = Batch::new(6);

        batch1.insert(job6);
        batch1.insert(job11);
        batch2.insert(job5);
        batch2.insert(job7);
        batch2.insert(job15);
        batch3.insert(job8);
        batch3.insert(job9);
        batch3.insert(job14);
        batch4.insert(job3);
        batch4.insert(job4);
        batch4.insert(job13);
        batch5.insert(job1);
        batch5.insert(job10);
        batch6.insert(job2);
        batch6.insert(job12);

        let mut schedule = BatchSchedule::new();

        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);
        schedule.insert_end(batch5);
        schedule.insert_end(batch6);

        let tardiness = get_tardiness(&schedule);
        assert_eq!(tardiness, 45);
    }
}
