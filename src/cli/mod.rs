use clap::{Arg, App,  ArgMatches, AppSettings};
mod sub;

pub fn en() -> ArgMatches<'static> {
  return App::new("pixelsort")
  .version("1.0")
  .setting(AppSettings::VersionlessSubcommands)
  .author("Nicolas JOULIN <joulin.nicolas@gmail.com>")
  .about("A small tool to do pixelsort stuff.")
  .arg(Arg::with_name("config")
    .short("c")
    .long("config")
    .value_name("FILE")
    .help("Sets a custom config file")
    .takes_value(true))
  .arg(Arg::with_name("INPUT")
    .help("Sets the path to the image to pixelsort")
    .required(true)
    .index(1))
  .arg(Arg::with_name("direction")
    .help(" default is left to right.")
    .long_help("Sets a direction in which to sort the picture. Can take one of four values :\n'l2r' : left to right\n'r2l' : right to left\n't2b' : top to bottom\n'b2t' : bottom to top\n")
    .short("d")
    .long("direction")
    .possible_values(&["l2r", "r2l", "t2b", "b2t"])
    .default_value("l2r"))
  .subcommand(sub::threshold())
  .subcommand(sub::mask())
  .get_matches();
}