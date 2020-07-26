use std::time::{SystemTime, UNIX_EPOCH};
use image::{Pixel};

pub fn generate_id() -> u128 {
  return SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time broke.")
    .as_millis();
}

pub fn lightness<T>(pixel: &&T) -> u8
where T: Pixel<Subpixel = u8> {
  pixel.to_luma()[0] as u8
}