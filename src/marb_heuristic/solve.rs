use std::fmt;

use crate::resources::BATCH_CAPACITY;
use crate::structures::Job;

#[derive(Debug, PartialEq, Clone)]
struct MarbBatch {
    code: usize,
    jobs: Vec<Job>,
    size: u32,
    processing_time: u32,
    attribute_ratio: f64,
}

impl MarbBatch {
    fn new(code: usize) -> Self {
        MarbBatch {
            code,
            jobs: Vec::new(),
            size: 0,
            processing_time: 0,
            attribute_ratio: 0.0,
        }
    }

    fn insert(&mut self, job: Job) {
        self.processing_time = self.processing_time.max(job.processing_time);
        self.size += job.size;
        self.jobs.push(job);
        self.attribute_ratio = self.processing_time as f64 / self.jobs.len() as f64;
    }
}

impl fmt::Display for MarbBatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Batch {}:", self.code)?;

        write!(f, "    jobs: ")?;
        for (i, job) in self.jobs.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", job.code)?;
        }

        writeln!(f)?;
        Ok(())
    }
}
pub(crate) fn solver(list: &mut Vec<Job>) -> Vec<MarbBatch> {
    if list.is_empty() {
        panic!("Empty SLUJ passed to solver");
    }

    let mut formed_batches: Vec<MarbBatch> = Vec::new();
    let mut batch = MarbBatch::new(1);

    Job::sort_due_date(list);
    batch.insert(list.pop().unwrap());
    formed_batches.push(batch);

    loop {
        if list.is_empty() {
            break;
        }
        solver_helper(&mut formed_batches, list.pop().unwrap());
    }

    formed_batches
}

fn solver_helper(formed_batches: &mut Vec<MarbBatch>, job: Job) {
    let eligible_batches: Vec<EligibleBatch> = get_eligible_batches(formed_batches, job.size);

    let mut min_ar = f64::MAX;
    let mut min_ar_index: usize = usize::MAX;

    for batch in eligible_batches {
        let new_ar = get_new_attribute_ratio(&formed_batches[batch.index], &job);
        if new_ar < batch.attribute_ratio && min_ar > new_ar {
            min_ar = new_ar;
            min_ar_index = batch.index;
        }
    }

    if min_ar == f64::MAX {
        let mut batch = MarbBatch::new(formed_batches.len() + 1);
        batch.insert(job);
        formed_batches.push(batch);
        return;
    }

    formed_batches[min_ar_index].insert(job);
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct EligibleBatch {
    index: usize,
    attribute_ratio: f64,
}

fn get_eligible_batches(formed_batches: &[MarbBatch], size: u32) -> Vec<EligibleBatch> {
    let mut result: Vec<EligibleBatch> = Vec::new();
    for (i, batch) in formed_batches.iter().enumerate() {
        if BATCH_CAPACITY - batch.size >= size {
            result.push(EligibleBatch {
                index: i,
                attribute_ratio: get_attribute_ratio(batch),
            });
        }
    }

    result
}

fn get_attribute_ratio(batch: &MarbBatch) -> f64 {
    batch.processing_time as f64 / batch.jobs.len() as f64
}

fn get_new_attribute_ratio(batch: &MarbBatch, job: &Job) -> f64 {
    let processing_time = batch.processing_time.max(job.processing_time) as f64;
    processing_time / (batch.jobs.len() as f64 + 1.0)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::resources::problem3::*;

    fn test_eligible_batches() {
        let job18 = job18();
        let job11 = job11();
        let job22 = job22();
        let job24 = job24();
        let job16 = job16();
        let job21 = job21();
        let job5 = job5();
        let job9 = job9();
        let job14 = job14();
        let job8 = job8();
        let job15 = job15();
        let job6 = job6();
        let job13 = job13();
        let job19 = job19();
        let job23 = job23();
        let job7 = job7();
        let job3 = job3();
        let job25 = job25();
        let job4 = job4();
        let job17 = job17();
        let tester = job1();
        // let tester = job20();

        let mut batch1 = MarbBatch::new(1);
        let mut batch2 = MarbBatch::new(2);
        let mut batch3 = MarbBatch::new(3);
        let mut batch4 = MarbBatch::new(4);
        let mut batch5 = MarbBatch::new(5);
        let mut batch6 = MarbBatch::new(6);
        let mut batch7 = MarbBatch::new(7);
        let mut batch8 = MarbBatch::new(8);

        batch1.insert(job18);
        batch1.insert(job19);
        batch1.insert(job5);
        batch2.insert(job11);
        batch2.insert(job22);
        batch2.insert(job21);
        batch3.insert(job24);
        batch3.insert(job16);
        batch4.insert(job9);
        batch4.insert(job14);
        batch4.insert(job8);
        batch5.insert(job15);
        batch5.insert(job6);
        batch5.insert(job23);
        batch6.insert(job13);
        batch6.insert(job7);
        batch6.insert(job3);
        batch7.insert(job25);
        batch7.insert(job4);
        batch8.insert(job17);

        let formed_batches = vec![
            batch1, batch2, batch3, batch4, batch5, batch6, batch7, batch8,
        ];
        let elig = get_eligible_batches(&formed_batches, tester.size);

        let solution = vec![EligibleBatch {
            index: 6,
            attribute_ratio: 4.5,
        },
            EligibleBatch {
                index: 7,
                attribute_ratio: 2.0
            }
        ];

        assert_eq!(elig, solution);

        let new_ar_at6 = get_new_attribute_ratio(&formed_batches[6], &tester);
        let new_ar_at7 = get_new_attribute_ratio(&formed_batches[7], &tester);
        assert_eq!(new_ar_at6, 3.0, "AR 6 FAILED");
        assert_eq!(new_ar_at7, 1.0, "AR 7 FAILED");
    }

    #[test]
    fn check_insertion() {
        let job18 = job18();
        let job11 = job11();
        let job22 = job22();
        let job24 = job24();
        let job16 = job16();
        let job21 = job21();
        let job5 = job5();
        let job9 = job9();
        let job14 = job14();
        let job8 = job8();
        let job15 = job15();
        let job6 = job6();
        let job13 = job13();
        let job19 = job19();
        let job23 = job23();
        let job7 = job7();
        let job3 = job3();
        let job25 = job25();
        let job4 = job4();
        let job17 = job17();
        let job1 = job1();
        let job20 = job20();

        let mut list = vec![
            job18, job11, job22, job24, job16, job21, job5, job9, job14, job8, job15, job6, job13,
            job19, job23, job7, job3, job25, job4, job17, job1, job20
        ];
        let result = solver(&mut list);

        for batch in result.iter() {
            println!("{}", batch);
            println!();
        }

        let mut batch1 = MarbBatch::new(1);
        let mut batch2 = MarbBatch::new(2);
        let mut batch3 = MarbBatch::new(3);
        let mut batch4 = MarbBatch::new(4);
        let mut batch5 = MarbBatch::new(5);
        let mut batch6 = MarbBatch::new(6);
        let mut batch7 = MarbBatch::new(7);
        let mut batch8 = MarbBatch::new(8);

        batch1.insert(job18);
        batch1.insert(job19);
        batch1.insert(job5);
        batch2.insert(job11);
        batch2.insert(job22);
        batch2.insert(job21);
        batch3.insert(job24);
        batch3.insert(job16);
        batch4.insert(job9);
        batch4.insert(job14);
        batch4.insert(job8);
        batch5.insert(job15);
        batch5.insert(job6);
        batch5.insert(job23);
        batch6.insert(job13);
        batch6.insert(job7);
        batch6.insert(job3);
        batch7.insert(job25);
        batch7.insert(job4);
        batch7.insert(job20);
        batch8.insert(job17);
        batch8.insert(job1);

        let formed_batches = vec![
            batch1, batch2, batch3, batch4, batch5, batch6, batch7, batch8,
        ];
        assert_eq!(result, formed_batches);
    }

    fn final_test() {
        let result = solver(&mut problem3());
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
        let job16 = job16();
        let job17 = job17();
        let job18 = job18();
        let job19 = job19();
        let job20 = job20();
        let job21 = job21();
        let job22 = job22();
        let job23 = job23();
        let job24 = job24();
        let job25 = job25();

        let mut batch1 = MarbBatch::new(1);
        let mut batch2 = MarbBatch::new(2);
        let mut batch3 = MarbBatch::new(3);
        let mut batch4 = MarbBatch::new(4);
        let mut batch5 = MarbBatch::new(5);
        let mut batch6 = MarbBatch::new(6);
        let mut batch7 = MarbBatch::new(7);
        let mut batch8 = MarbBatch::new(8);
        let mut batch9 = MarbBatch::new(9);

        batch1.insert(job18);
        batch1.insert(job5);
        batch1.insert(job19);
        batch2.insert(job11);
        batch2.insert(job22);
        batch2.insert(job21);
        batch3.insert(job24);
        batch3.insert(job16);
        batch4.insert(job9);
        batch4.insert(job14);
        batch4.insert(job8);
        batch5.insert(job15);
        batch5.insert(job6);
        batch5.insert(job23);
        batch6.insert(job13);
        batch6.insert(job7);
        batch6.insert(job3);
        batch7.insert(job25);
        batch7.insert(job4);
        batch7.insert(job20);
        batch8.insert(job17);
        batch8.insert(job1);
        batch8.insert(job10);
        batch9.insert(job12);
        batch9.insert(job2);

        let formed_batches = vec![
            batch1, batch2, batch3, batch4, batch5, batch6, batch7, batch8, batch9,
        ];

        assert_eq!(result, formed_batches);
    }
}
