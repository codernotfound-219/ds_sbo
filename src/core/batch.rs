use crate::core::job::Job;
use crate::core::DecisionLog;
use std::fmt;
use std::u32::MAX;

#[derive(Clone, Debug)]
pub struct Batch {
    pub jobs: Vec<Job>,
    pub code: usize,
    pub release_date: u32,
    pub processing_time: u32,
    pub completion_time: u32,
    pub min_due_time: u32,
    pub size: u32,
    pub log: Option<DecisionLog>,
}

impl Batch {
    pub fn new(code: usize) -> Self {
        Batch {
            jobs: Vec::new(),
            code,
            release_date: 0,
            processing_time: 0,
            completion_time: 0,
            min_due_time: MAX,
            size: 0,
            log: None,
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
                self.jobs[0].release_date,
                self.jobs[0].processing_time,
                self.jobs[0].due_date,
                self.jobs[0].size,
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

        writeln!(f, "")?;
        writeln!(f, "    releaseDate: {}", self.release_date)?;
        writeln!(f, "    processingTime: {}", self.processing_time)?;
        writeln!(f, "    completionTime: {}", self.completion_time)?;
        writeln!(f, "    size: {}", self.size)?;

        Ok(())
    }
}
