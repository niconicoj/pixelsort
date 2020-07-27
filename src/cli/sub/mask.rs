use clap::{App, Arg, SubCommand};

pub fn mask() -> App<'static, 'static> {
  return 
    SubCommand::with_name("mask")
      .about("Intervals are defined by an other picture. the masking picture needs to be black and white for best result.")
      .arg(Arg::with_name("mask")
        .short("m")
        .long("mask")
        .required(true)
        .takes_value(true)
        .help("The path of the masking image."))
}

