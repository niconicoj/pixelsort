use std::time::{SystemTime, UNIX_EPOCH};
use palette::*;

pub fn generate_id() -> u128 {
  return SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time broke.")
    .as_millis();
}

pub fn lightness(pixel: &[u8; 3]) -> f32 {
  return  rgb_to_hsv(pixel).value;
}

fn rgb_to_hsv(pixel: &[u8]) -> Hsv {
  let red = pixel[0] as f32 / 255.0;
  let green = pixel[1] as f32 / 255.0;
  let blue = pixel[2] as f32 /255.0;

  return Srgb::new(red, green, blue).into_hsv();
}