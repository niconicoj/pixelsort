extern crate clap;
extern crate image;

mod cli;
mod utils;
mod intervals;
mod sorter;

fn main() {
  let matches = cli::en();
  let image_input_path = matches.value_of("INPUT").unwrap();
  let image_output_path = format!("{}{}", utils::generate_id(), ".png");
  println!("opening image...");
  let mut img = image::open(image_input_path).unwrap();

  img = match matches.value_of("direction").unwrap() {
    "l2r" => img,
    "r2l" => img.rotate180(),
    "t2b" => img.rotate270(), 
    "b2t" => img.rotate90(),
    _ => panic!("could not determine in which direction to sort the image.")
  };

  let buf = img.to_rgb();

  let mut output_img = match matches.subcommand_name() {
    Some("threshold") => {
      let lower = matches.subcommand_matches("threshold")
        .unwrap().value_of("lower").unwrap().parse().unwrap();
      let upper = matches.subcommand_matches("threshold")
        .unwrap().value_of("upper").unwrap().parse().unwrap();
      println!("computing intervals...");
      let intervals = intervals::threshold(&buf, lower, upper);
      println!("sorting pixels...");
      image::DynamicImage::ImageRgb8(sorter::sort(buf, intervals))
    },
    _ => panic!("You need to provide an interval function. For more information use --help.")
  };


  output_img = match matches.value_of("direction").unwrap() {
    "l2r" => output_img,
    "r2l" => output_img.rotate180(),
    "t2b" => output_img.rotate90(), 
    "b2t" => output_img.rotate270(),
    _ => panic!("could not determine in which direction to sort the image.")
  };
  println!("writing output image...");
  output_img.save(image_output_path).expect("something went wrong when saving file.");
}