use std::collections::HashMap;

pub struct BabyBlueTrainer {
    matrix: Vec<u32>,
    size: usize,
    char_map: HashMap<char, usize>,
    index_to_char: Vec<char>,
}

impl BabyBlueTrainer {
    pub fn new(unique_chars: &[char]) -> Self {
        let size = unique_chars.len();
        let mut char_map = HashMap::new();
        let index_to_char = unique_chars.to_vec();
        
        for (idx, &c) in unique_chars.iter().enumerate() {
            char_map.insert(c, idx);
        }
        
        Self {
            matrix: vec![0; size * size],
            size,
            char_map,
            index_to_char,
        }
    }

    pub fn char_to_index(&self, c: char) -> Option<usize> {
        self.char_map.get(&c).copied()
    }

    pub fn index_to_char(&self, idx: usize) -> Option<char> {
        if idx < self.index_to_char.len() {
            Some(self.index_to_char[idx])
        } else {
            None
        }
    }

    // only increments weights
    pub fn learn(&mut self, prev: usize, current: usize) {
        let index = prev * self.size + current;
        self.matrix[index] += 1;
    }

    // Provides weights for inference or storage
    pub fn export_matrix(self) -> Vec<u32> {
        self.matrix
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_char_map(&self) -> &HashMap<char, usize> {
        &self.char_map
    }

    pub fn get_index_to_char(&self) -> &[char] {
        &self.index_to_char
    }
}

// deprecated: use BabyBlueTrainer::char_to_index instead
pub fn char_to_index(c: char) -> Option<usize> {
    match c {
        'a'..='z' => Some((c as u8 - b'a') as usize),
        'A'..='Z' => Some((c as u8 - b'A') as usize),
        ' ' => Some(30),
        _ => None,
    }
}
