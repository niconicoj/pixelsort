use image::*;
use crate::utils;

pub fn sort<I>(img: &I) -> ImageBuffer<Rgb<u8>, Vec<u8>>
  where I: GenericImage<Pixel=Rgb<u8>> {
  
    let mut out = ImageBuffer::new(img.dimensions().0, img.dimensions().1);
    let mut data = vec![];

    for y in 0..img.height() {
      let mut row = vec![];
      for x in 0..img.width() {
        row.push(img.get_pixel(x, y).0);
      }
      row.sort_by(|a, b| utils::lightness(a).partial_cmp(&utils::lightness(b)).unwrap());
      data.push(row);
    }

    for (x,y,pixel) in out.enumerate_pixels_mut() {
      *pixel = image::Rgb(data[y as usize][x as usize]);
    }
    out
}