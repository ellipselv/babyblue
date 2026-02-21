use rand::Rng;

pub struct BabyBlueInference {
    matrix: Vec<u32>,
    size: usize,
}

impl BabyBlueInference {
    pub fn from_weights(weights: Vec<u32>, size: usize) -> Self {
        Self {
            matrix: weights,
            size,
        }
    }

    pub fn predict(&self, current: usize) -> usize {
        let start = current * self.size;
        let row = &self.matrix[start..start + self.size];

        // Finds the max weight (simple greedy selection).
        row.iter()
            .enumerate()
            .max_by_key(|&(_, &val)| val)
            .map(|(idx, _)| idx)
            .unwrap_or(30) // Defaults to space.
    }

    pub fn predict_creative(&self, current: usize) -> usize {
        let start = current * self.size;
        let row = &self.matrix[start..start + self.size];
        
        let sum: u32 = row.iter().sum();
        
        if sum == 0 { return 30; }

        let mut rng = rand::thread_rng();
        let mut target = rng.gen_range(0..sum);

        for (idx, &weight) in row.iter().enumerate() {
            if target < weight {
                return idx;
            }
            target -= weight;
        }
        
        30
    }
}
