#[cfg(test)]
mod test {
    use ds_sbo_rust::greedy_dp::deviation_calculator::insertion;
    use ds_sbo_rust::greedy_dp::{get_action, get_creation_deviations, get_insertion_deviations};
    use ds_sbo_rust::resources::problem2::*;
    use ds_sbo_rust::structures::{BatchSchedule, Batch};

    #[test]
    fn check_prob3_solution() {
        let job5= job5();
        let job2 = job2();
        let job1 = job1();
        let job12 = job12();
        let job10 = job10();
        let job8 = job8();
        let job11 = job11();
        let job4 = job4();
        let job7 = job7();
        let job6 = job6();
        let job9 = job9();
        let job14 = job14();
        let job3 = job3();
        let job13 = job13();
        let tester = job15();

        let mut batch1 = Batch::new(1);
        let mut batch2 = Batch::new(2);
        let mut batch3 = Batch::new(3);
        let mut batch4 = Batch::new(4);
        let mut batch5 = Batch::new(5);
        let mut batch6 = Batch::new(6);
        let mut batch7 = Batch::new(7);

        batch1.insert(job5);
        batch1.insert(job2);
        batch2.insert(job10);
        batch2.insert(job11);
        batch2.insert(job8);
        batch3.insert(job12);
        batch3.insert(job1);
        batch4.insert(job4);
        batch4.insert(job7);
        batch5.insert(job6);
        batch5.insert(job9);
        batch6.insert(job14);
        batch6.insert(job3);
        batch7.insert(job13);

        let mut schedule = BatchSchedule::new();
        schedule.insert_end(batch1);
        schedule.insert_end(batch2);
        schedule.insert_end(batch3);
        schedule.insert_end(batch4);
        schedule.insert_end(batch5);
        schedule.insert_end(batch6);
        schedule.insert_end(batch7);

        let insertion = get_insertion_deviations(&schedule, &tester);
        let creation = get_creation_deviations(&schedule, &tester);
        let best_action = get_action(&insertion, &creation);

        println!("{:#?}", insertion);
        println!("{:#?}", creation);
        println!("{:#?}", best_action);
        assert_eq!(true, false);
    }
}
