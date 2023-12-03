use std::io::Error;

mod constants;
mod part_1;
mod part_2;

use crate::utils::read_input;
use constants::{INPUT_FILENAME, PART_2_EXAMPLE_INPUT};

// part 1 too low: 36819
// part 1 too low: 37078
// part 1 correct: 55123
pub fn run() -> Result<(), Error> {
  println!("reading contents of {}", INPUT_FILENAME);
  let contents = read_input(INPUT_FILENAME)?;

  let part_1_total = part_1::run(&contents);
  println!("part_1 total {}", part_1_total);
  let part_2_total = part_2::run(&contents);
  println!("part_2 total {}", part_2_total);

  Ok(())
}
