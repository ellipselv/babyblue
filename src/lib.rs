pub mod inference;
pub mod trainer;

pub use inference::BabyBlueInference;
pub use trainer::BabyBlueTrainer;

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn save_raw_weights<P: AsRef<Path>>(path: P, weights: &[u32]) -> io::Result<()> {
	let mut buf = Vec::with_capacity(weights.len() * 4);
	for &value in weights {
		buf.extend_from_slice(&value.to_le_bytes());
	}

	let mut file = File::create(path)?;
	file.write_all(&buf)
}

pub fn load_raw_weights<P: AsRef<Path>>(path: P) -> io::Result<Vec<u32>> {
	let mut file = File::open(path)?;
	let mut buf = Vec::new();
	file.read_to_end(&mut buf)?;

	if buf.len() % 4 != 0 {
		return Err(io::Error::new(
			io::ErrorKind::InvalidData,
			"raw weights length is not a multiple of 4",
		));
	}

	let mut weights = Vec::with_capacity(buf.len() / 4);
	for chunk in buf.chunks_exact(4) {
		let value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
		weights.push(value);
	}

	Ok(weights)
}
