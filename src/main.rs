use babyblue::{
    load_raw_weights, save_raw_weights, BabyBlueInference, BabyBlueTrainer,
};
use std::fs;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let text = fs::read_to_string("input.txt")
        .expect("input.txt not found");

    println!("Total symbols: {}", text.len());

    // find all unique characters
    let mut unique_chars: Vec<char> = text.chars()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    unique_chars.sort();

    println!("Unique symbols: {}", unique_chars.len());
    println!("Symbols: {:?}", unique_chars);

    // create trainer with dynamic size
    let mut trainer = BabyBlueTrainer::new(&unique_chars);
    
    let mut prev_idx: Option<usize> = None;

    // train on the text
    for c in text.chars() {
        if let Some(current_idx) = trainer.char_to_index(c) {
            if let Some(p) = prev_idx {
                trainer.learn(p, current_idx);
            }
            prev_idx = Some(current_idx);
        }
    }
    
    let size = trainer.get_size();
    let index_to_char = trainer.get_index_to_char().to_vec();
    
    let weights = trainer.export_matrix();
    save_raw_weights("weights.bin", &weights)?;

    let loaded_weights = load_raw_weights("weights.bin")?;
    let predictor = BabyBlueInference::from_weights(loaded_weights, size);
    
    let mut current_idx = index_to_char.get(0).and_then(|_| {
        // start with first character in our alphabet
        Some(0)
    }).unwrap_or(0);

    println!("\nGenerated text:");
    for _ in 0..100 {
        let next_idx = predictor.predict_creative(current_idx);
        if let Some(next_char) = index_to_char.get(next_idx) {
            print!("{}", next_char);
        }
        current_idx = next_idx;
    }
        
    println!();

    Ok(())
}