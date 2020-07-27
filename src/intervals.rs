use image::*;
use std::ops::{Range};
use crate::utils;

pub fn threshold(img: &RgbImage, lower_threshold: u8, upper_threshold: u8) -> Vec<Vec<Range<usize>>> {
  let mut result: Vec<Vec<Range<usize>>> = vec![];

  for (_, row) in img.rows().enumerate() {
    let mut row_vec = vec![];
    let mut flag = (false, 0);
    for (x, pixel) in row.enumerate() {
      if flag.0 {
        if !(lower_threshold..upper_threshold).contains(&utils::lightness(&pixel)) {
          row_vec.push(flag.1..x);
          flag.0 = false;
        }
      } else {
        if (lower_threshold..upper_threshold).contains(&utils::lightness(&pixel)) {
          flag = (true, x);
        }
      }
    }
    if flag.0 {
      row_vec.push(flag.1..img.width() as usize)
    }
    result.push(row_vec);
  }
  result
}

pub fn mask(mask: &RgbImage) -> Vec<Vec<Range<usize>>> {
  let mut result: Vec<Vec<Range<usize>>> = vec![];

  for (_, row) in mask.rows().enumerate() {
    let mut row_vec = vec![];
    let mut flag = (false, 0);
    for (x, pixel) in row.enumerate() {
      if flag.0 {
        if pixel.0 == [0,0,0] {
          row_vec.push(flag.1..x);
          flag.0 = false;
        }
      } else {
        if pixel.0 == [255,255,255] {
          flag = (true, x);
        }
      }
    }
    if flag.0 {
      row_vec.push(flag.1..mask.width() as usize)
    }
    result.push(row_vec);
  }
  result
}