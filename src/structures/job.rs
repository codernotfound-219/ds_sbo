use std::fmt;
use std::cmp::Reverse;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Job {
    pub code: u32,
    pub release_date: u32,
    pub due_date: u32,
    pub processing_time: u32,
    pub size: u32,
}

impl Job {
    pub(crate) fn new(code: u32, rel:u32, due: u32, pt: u32, size: u32) -> Job {
        Job {
            code,
            release_date: rel,
            due_date: due,
            processing_time: pt,
            size,
        }
    }

    pub fn sort_release_date(list: &mut [Job]) {
        list.sort_by_key(|job| Reverse(job.release_date));
    }

    pub fn sort_due_date(list: &mut [Job]) {
        list.sort_by(|a, b| {
            Reverse(a.due_date).cmp(&Reverse(b.due_date))
                .then(Reverse(a.code).cmp(&Reverse(b.code)))
        });
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        self.due_date
            .cmp(&other.due_date)
            .then(self.processing_time.cmp(&other.processing_time))
            .then(self.size.cmp(&other.size))
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
