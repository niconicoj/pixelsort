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
  .subcommand(sub::threshold())
  .get_matches();
}