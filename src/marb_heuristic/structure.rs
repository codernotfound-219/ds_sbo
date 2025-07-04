use std::fmt;
use crate::structures::Job;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EligibleBatch {
    pub index: usize,
    pub attribute_ratio: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MarbBatch {
    pub code: usize,
    pub jobs: Vec<Job>,
    pub size: u32,
    pub processing_time: u32,
    pub attribute_ratio: f64,
}

impl MarbBatch {
    pub fn new(code: usize) -> Self {
        MarbBatch {
            code,
            jobs: Vec::new(),
            size: 0,
            processing_time: 0,
            attribute_ratio: 0.0,
        }
    }

    pub fn insert(&mut self, job: Job) {
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
