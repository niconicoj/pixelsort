extern crate clap;
extern crate image;

mod cli;
mod utils;
mod intervals;

fn main() {
    let matches = cli::en();
    let image_input_path = matches.value_of("INPUT").unwrap();
    let image_output_path = format!("{}{}", utils::generate_id(), ".png");
    println!("opening image {}", image_input_path);
    let img = image::open(image_input_path).unwrap().into_rgb();
    
    match matches.subcommand_name() {
      Some("threshold") => {
       println!("using threshold");
       let intervals = intervals::threshold(&img, 0.25, 0.8);
       println!("intervals : {:?}", intervals[0]);
      },
      _ => {},
    }
}