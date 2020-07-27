use clap::{App};
mod threshold;
mod mask;

pub fn threshold() -> App<'static, 'static> {
  threshold::threshold()
}

pub fn mask() -> App<'static, 'static> {
  mask::mask()
}