use image::{Rgb, RgbImage, ImageBuffer, Pixel};
use std::ops::Range;
use crate::utils::lightness;

pub fn sort(img: RgbImage, intervals: Vec<Range<usize>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  let mut pixels: Vec<_> = img.pixels().collect();

  for interval in intervals.into_iter() {
    pixels[interval].sort_by(|a,b| lightness(a).cmp(&lightness(b)));
  }
  let mut buf = ImageBuffer::new(img.width(), img.height());

  for (i, buf_p) in buf.pixels_mut().enumerate() {
    *buf_p = *pixels[i];
  }

  buf
}