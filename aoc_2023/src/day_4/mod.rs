use crate::utils::read_input;

const EXAMPLE_INPUT_FILENAME: &str = "./src/day_4/example_input.txt";
const INPUT_FILENAME: &str = "./src/day_4/input.txt";

pub fn part_1(contents: &str) -> u32 {
  0
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 4");
  println!("reading contents of {}", INPUT_FILENAME);
  let contents = read_input(INPUT_FILENAME)?;

  let part_1_total = part_1(&contents);
  println!("part_1 total {}", part_1_total);

  // let part_2_total = part_2(&contents);
  // println!("part_2 total {}", part_2_total);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::{part_1, EXAMPLE_INPUT_FILENAME, INPUT_FILENAME};
  use crate::utils::read_input;

  #[test]
  pub fn day_4_part_1_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 13);
  }

  /*
  #[test]
  pub fn day_3_part_1_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    // assert_eq!(result, 4361);
  }
  */
}
