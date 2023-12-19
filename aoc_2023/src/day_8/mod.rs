pub const INPUT_FILENAME: &str = "src/day_8/input.txt";

fn part_1(input: &str) -> u64 {
  0
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 7");
  let contents = crate::utils::read_input(INPUT_FILENAME)?;

  let part_1_total = part_1(&contents);
  println!("part_1 total {}", part_1_total);

  // let part_2_total = part_2(&contents);
  // println!("part_2 total {}", part_2_total);

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::utils::read_input;

  use super::{part_1, part_2, INPUT_FILENAME};

  const EXAMPLE_INPUT_FILENAME: &str = "./src/day_7/example_input.txt";

  #[test]
  pub fn day_8_part_1_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 6);
  }

  // #[test]
  // pub fn day_8_part_1_solution_works() {
  //   let contents = read_input(INPUT_FILENAME).unwrap();
  //   let result = part_1(&contents);
  //   assert_eq!(result, 247815719);
  // }

  // #[test]
  // pub fn day_8_part_2_example_works() {
  //   let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
  //   let result = part_2(&contents);
  //   assert_eq!(result, 5905);
  // }

  // #[test]
  // pub fn day_8_part_2_solution_works() {
  //   use more_asserts::assert_gt;

  //   let contents = read_input(INPUT_FILENAME).unwrap();
  //   let result = part_2(&contents);
  //   assert_gt!(result, 248673364);
  //   assert_eq!(result, 248747492);
  // }
}
