use std::io::Error;

mod constants;
mod part_1;

use crate::utils::read_input;
use constants::{INPUT_FILENAME, PART_1_EXAMPLE_INPUT_FILENAME};

pub fn run() -> Result<(), Error> {
  println!("reading contents of {}", PART_1_EXAMPLE_INPUT_FILENAME);
  let contents = read_input(PART_1_EXAMPLE_INPUT_FILENAME)?;

  let part_1_total = part_1::run(&contents);
  println!("part_1 total {}", part_1_total);

  Ok(())
}
