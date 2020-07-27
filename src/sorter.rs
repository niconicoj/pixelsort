use image::RgbImage;
use std::ops::Range;
use crate::utils::lightness;

pub fn sort(img: RgbImage, intervals: Vec<Vec<Range<usize>>>) -> RgbImage {
  let mut pixels: std::vec::Vec<&image::Rgb<u8>> = vec![];

  for (row, row_interval) in img.rows().zip(intervals) {
    let mut row_pixel: Vec<_> = row.collect();
    for interval in row_interval.into_iter() {
      row_pixel[interval].sort_by(|a,b| lightness(a).cmp(&lightness(b)));
    }
    pixels.append(&mut row_pixel);
  }

  let mut buf = RgbImage::new(img.width(), img.height());
  for (i, buf_p) in buf.pixels_mut().enumerate() {
    *buf_p = *pixels[i];
  }
  buf
}