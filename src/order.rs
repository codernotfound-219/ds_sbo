use crate::batch::Batch;

pub struct Order {
    pub batches: Vec<Batch>,
}

impl Order {
    pub fn new() -> Self {
        Order {
            batches: Vec::new(),
        }
    }
    
    pub fn get_batches(&self) -> &Vec<Batch> {
        &self.batches
    }

    pub fn add_batch(&mut self, batch: Batch) {
        self.batches.push(batch);
        let batch_index = self.batches.len() - 1;
        self.update_parameters(batch_index);
    }

    fn update_parameters(&mut self, batch_num: usize) {
        let prev_completion = if batch_num == 0 {
            0
        } else {
            self.batches[batch_num - 1].completion_time
        };

        let batch = &mut self.batches[batch_num];
        batch.release_date = batch.release_date.max(prev_completion);
        batch.completion_time = batch.release_date + batch.processing_time;
    }
}
