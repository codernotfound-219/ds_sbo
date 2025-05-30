use crate::job::Job;
use std::fmt;

#[derive(Debug)]
pub struct Batch {
    pub jobs: Vec<Job>,
    pub code: u32,
    pub release_date: u32,
    pub processing_time: u32,
    pub completion_time: u32,
    pub size: u32,
}

impl Batch {
    pub fn new(code: u32, jobs: Vec<Job>) -> Self {
        // let size = jobs.iter()
        //     .map(|job| job.size)
        //     .sum();
        //
        // let release_date = jobs.iter()
        //     .map(|job| job.release_date)
        //     .max()
        //     .unwrap_or(0);
        //
        // let processing_time = jobs.iter()
        //     .map(|job| job.processing_time)
        //     .max()
        //     .unwrap_or(0);
        //
        // let completion_time = release_date + processing_time;

        //NOTE: the fold() does the job of O(3n) in O(n)
        let (release_date, processing_time, size) =
            jobs.iter()
                .fold((0, 0, 0), |(max_rel, max_pro, total_size), job| {
                    (
                        max_rel.max(job.release_date),
                        max_pro.max(job.processing_time),
                        total_size + job.size,
                    )
                });

        Batch {
            jobs,
            code,
            release_date,
            processing_time,
            completion_time: release_date + processing_time,
            size,
        }
    }

    pub fn insert_end(&mut self, job: Job) {
        self.jobs.push(job);
    }

    pub fn insert_begin(&mut self, job: Job) {
        self.jobs.insert(0, job);
    }

    pub fn insert_at_position(&mut self, index: usize, job: Job) {
        self.jobs.insert(index, job);
    }

    pub fn pop_job(&mut self) -> Option<Job> {
        self.jobs.pop()
    }
}

impl fmt::Display for Batch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Batch {}:", self.code)?;

        write!(f, "    jobs: ")?;
        for (i, job) in self.jobs.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
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
