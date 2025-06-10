use crate::core::job::Job;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Batch {
    pub jobs: Vec<Job>,
    pub code: usize,
    pub release_date: u32,
    pub processing_time: u32,
    pub completion_time: u32,
    pub min_due_time: u32,
    pub size: u32,
}

impl Batch {
    pub fn new(code: usize) -> Self {
        Batch {
            jobs: Vec::new(),
            code,
            release_date: 0,
            processing_time: 0,
            completion_time: 0,
            min_due_time: u32::MAX,
            size: 0,
        }
    }

    pub fn insert_end(&mut self, job: Job) {
        self.jobs.push(job);
        self.update_batch_param();
    }

    pub fn insert_begin(&mut self, job: Job) {
        self.jobs.insert(0, job);
        self.update_batch_param();
    }

    pub fn insert_at_position(&mut self, index: usize, job: Job) {
        self.jobs.insert(index, job);
        self.update_batch_param();
    }

    pub fn pop_job(&mut self) -> Option<Job> {
        let out = self.jobs.pop();
        self.update_batch_param();
        out
    }

    fn update_batch_param(&mut self) {
        let (release_date, processing_time, min_due_time, size) = self.jobs.iter().fold(
            (
                0,
                0,
                u32::MAX,
                0,
            ),
            |(max_rel, max_pro, min_due, total_size), job| {
                (
                    max_rel.max(job.release_date),
                    max_pro.max(job.processing_time),
                    min_due.min(job.due_date),
                    total_size + job.size,
                )
            },
        );

        self.release_date = release_date;
        self.processing_time = processing_time;
        self.min_due_time = min_due_time;
        self.completion_time = self.release_date + self.processing_time;
        self.size = size;
    }
}

impl fmt::Display for Batch {
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
        writeln!(f, "    releaseDate: {}", self.release_date)?;
        writeln!(f, "    processingTime: {}", self.processing_time)?;
        writeln!(f, "    completionTime: {}", self.completion_time)?;
        writeln!(f, "    size: {}", self.size)?;

        Ok(())
    }
}
