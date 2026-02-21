use babyblue::{
    char_to_index, load_raw_weights, save_raw_weights, BabyBlueInference, BabyBlueTrainer,
};
use std::fs;

fn main() -> std::io::Result<()> {
    let size = 31;
    
    let mut trainer = BabyBlueTrainer::new(size);
    
    let text = fs::read_to_string("input.txt")
        .expect("input.txt not found");

    println!("Symbols: {}", text.len());

    let mut prev_idx: Option<usize> = None;

    for c in text.chars() {
        if let Some(current_idx) = char_to_index(c) {
            if let Some(p) = prev_idx {
                trainer.learn(p, current_idx);
            }
            prev_idx = Some(current_idx);
        }
    }
    
    let weights = trainer.export_matrix();
    save_raw_weights("weights.bin", &weights)?;

    let loaded_weights = load_raw_weights("weights.bin")?;
    let predictor = BabyBlueInference::from_weights(loaded_weights, size);
    
    let mut current_idx = char_to_index('a').unwrap();

    for _ in 0..50 {
        let next_idx = predictor.predict_creative(current_idx);
        let next_char = match next_idx {
            30 => ' ',
            i @ 0..=25 => (i as u8 + b'a') as char,
            _ => '?',
        };
        print!("{}", next_char);
        current_idx = next_idx;
    }
        
    println!();

    Ok(())
}