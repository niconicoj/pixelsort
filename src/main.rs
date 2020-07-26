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
    println!("l? {}, u? {}", matches.is_present("LOWER_THRESHOLD"), matches.is_present("UPPER_THRESHOLD"));
    println!("opening image {}", image_input_path);
    let img = image::open(image_input_path).unwrap().rotate270().to_rgb();

  if let Some(matches) = matches.subcommand_matches("threshold") {
    println!("using threshold");
    let lower = if matches.is_present("lower") {
      matches.value_of("lower").unwrap().parse().expect("lower threshold has to be an integer between 0 and 255")
    } else {
      32
    };
    let upper = if matches.is_present("upper") {
      matches.value_of("upper").unwrap().parse().expect("upper threshold has to be an integer between 0 and 255")
    } else {
      223
    };
    println!("lower threshold : {}, upper threshold : {}", lower, upper);
    let intervals = intervals::threshold(&img, lower, upper);
    let output_img = image::DynamicImage::ImageRgb8(sorter::sort(img, intervals));

    output_img.rotate90().save(image_output_path).expect("something went wrong when saving file.");
  }
}