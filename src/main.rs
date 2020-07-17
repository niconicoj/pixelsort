extern crate clap;
extern crate image;

mod cli;
mod utils;
mod sorter;

fn main() {
    let matches = cli::en();
    let image_input_path = matches.value_of("INPUT").unwrap();
    let image_output_path = format!("{}{}", utils::generate_id(), ".png");
    println!("opening image {}", image_input_path);
    let img = image::open(image_input_path).unwrap().into_rgb();


    
    match matches.subcommand_name() {
      Some("threshold") => {
       println!("using threshold");
       let output_img = sorter::sort(&img);

       output_img.save(image_output_path).expect("something went wrong when saving file.");

      },
      _ => {},
    }
}