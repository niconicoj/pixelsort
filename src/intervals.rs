use image::*;
use crate::utils;

pub fn threshold(img: &RgbImage, lower_threshold: f32, upper_threshold: f32) -> Vec<Vec<u32>> {
  let mut intervals = vec![];
  for y in 0..img.height() {
    let mut interval = vec![];
    for x in 0..img.width() {
      let pixel = img.get_pixel(x,y).0;
      let level = utils::lightness(&pixel);
      if  (level < lower_threshold) | (level > upper_threshold) {
        interval.push(x);
      }
    }
    intervals.push(interval);
  }
  return intervals;
}