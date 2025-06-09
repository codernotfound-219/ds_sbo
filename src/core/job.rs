use std::fmt;
use std::cmp::Reverse;

#[derive(Clone, Debug)]
pub struct Job {
    pub code: u32,
    pub release_date: u32,
    pub due_date: u32,
    pub processing_time: u32,
    pub size: u32,
}

impl Job {
    pub fn new(code: u32, rel: u32, due: u32, pt: u32, size: u32) -> Job {
        Job {
            code,
            release_date: rel,
            due_date: due,
            processing_time: pt,
            size,
        }
    }

    pub fn sort_release_date(list: &mut Vec<Job>) {
        list.sort_by_key(|job| Reverse(job.release_date));
    }
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Job {}:", self.code)?;
        writeln!(f, "    releaseDate: {}", self.release_date)?;
        writeln!(f, "    processingTime: {}", self.processing_time)?;
        writeln!(f, "    dueDate: {}", self.due_date)?;
        writeln!(f, "    size: {}", self.size)?;

        Ok(())
    }
}
