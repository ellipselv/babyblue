pub struct BabyBlueTrainer {
    matrix: Vec<u32>,
    size: usize,
}

pub fn char_to_index(c: char) -> Option<usize> {
    match c {
        'a'..='z' => Some((c as u8 - b'a') as usize),
        'A'..='Z' => Some((c as u8 - b'A') as usize),
        ' ' => Some(30),
        _ => None,
    }
}

impl BabyBlueTrainer {
    pub fn new(size: usize) -> Self {
        Self {
            matrix: vec![0; size * size],
            size,
        }
    }

    // Only increments weights
    pub fn learn(&mut self, prev: usize, current: usize) {
        let index = prev * self.size + current;
        self.matrix[index] += 1;
    }

    // Provides weights for inference or storage
    pub fn export_matrix(self) -> Vec<u32> {
        self.matrix
    }
}
