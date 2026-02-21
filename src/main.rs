struct BabyBlue {
    matrix: Vec<u32>,
    alphabed_size: usize,
}

impl BabyBlue {
    fn new(alphabed_size: usize) -> Self {
        Self {
            matrix: vec![0; alphabed_size * alphabed_size],
            alphabed_size,
        }
    }

    fn increment(&mut self, prev: usize, current: usize) {
        let index = prev * self.alphabed_size + current;
        self.matrix[index] += 1;
    }
}

fn char_to_index(c: char) -> Option<usize> {
    match c.to_ascii_lowercase() {
        'a'..='z' => Some((c as u8 - b'a') as usize),
        ' ' => Some(26),
        _ => None,
    }
}

fn main() {
    let mut model = BabyBlue::new(31);
    let text = "baby blue";

    let mut prev_idx: Option<usize> = None;

    for c in text.chars() {
        if let Some(current_idx) = char_to_index(c) {
            if let Some(p) = prev_idx {
                model.increment(p, current_idx);
            }

            prev_idx = Some(current_idx);
        }
    }
}