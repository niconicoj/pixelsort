use clap::{App, Arg, SubCommand};

pub fn threshold() -> App<'static, 'static> {
  return 
    SubCommand::with_name("threshold")
      .about("Intervals defined by lightness thresholds; only pixels with a lightness outside of the upper and lower thresholds interval are sorted.")
      .arg(Arg::with_name("lower")
        .short("l")
        .long("lower")
        .help("How dark must a pixel be to be considered as a 'border' for sorting. Takes values from 0-1. 0.25 by default.")
        .takes_value(true)
        .default_value("32"))
      .arg(Arg::with_name("upper")
        .short("u")
        .long("upper")
        .help("How bright must a pixel be to be considered as a 'border' for sorting. Takes values from 0-1. 0.8 by default.")
        .takes_value(true)
        .default_value("224"))
}
