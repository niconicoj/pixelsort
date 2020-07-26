use image::*;
use std::ops::{Range};
use crate::utils;

pub fn threshold(img: &RgbImage, lower_threshold: u8, upper_threshold: u8) -> Vec<Range<usize>> {
  let mut result: Vec<Range<usize>> = vec![];
  let mut flag = (false, 0);

  for (i,p) in img.pixels().enumerate() {
    if flag.0 {
      if !(lower_threshold..upper_threshold).contains(&utils::lightness(&p)) {
        result.push(flag.1..i);
        flag.0 = false;
      }
    } else {
      if (lower_threshold..upper_threshold).contains(&utils::lightness(&p)) {
        flag = (true,i);
      }
    }
  }
  if flag.0 {
    result.push(flag.1..img.pixels().len());
  }
  result
}