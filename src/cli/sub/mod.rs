use clap::{App};
mod threshold;

pub fn threshold() -> App<'static, 'static> {
  return threshold::threshold();
}