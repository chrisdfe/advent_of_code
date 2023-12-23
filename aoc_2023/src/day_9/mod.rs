const INPUT_FILENAME: &str = "./src/day_9/input.txt";

fn part_1(input: &str) -> u64 {
  println!("input: {}", input);
  0
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 9");
  let input = crate::utils::read_input(INPUT_FILENAME)?;

  let part_1_total = part_1(&input);
  println!("part_1 total {}", part_1_total);

  // let part_2_total = part_2(&input);
  // println!("part_2 total {}", part_2_total);

  Ok(())
}

#[cfg(test)]
mod tests {
  use more_asserts::assert_gt;

  use crate::utils::read_input;

  use super::part_1;

  const EXAMPLE_INPUT_FILENAME: &str = "./src/day_9/example_input.txt";

  #[test]
  pub fn day_8_part_1_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 114);
  }

  // #[test]
  // pub fn day_8_part_1_solution_works() {
  //   let contents = read_input(INPUT_FILENAME).unwrap();
  //   let result = part_1(&contents);
  //   assert_eq!(result, 12737);
  // }

  // #[test]
  // pub fn day_8_part_2_example_works() {
  //   let contents = read_input(PART_2_EXAMPLE_INPUT_FILENAME).unwrap();
  //   let result = part_2(&contents);
  //   assert_eq!(result, 6);
  // }

  // #[test]
  // pub fn day_8_part_2_solution_works() {
  //   use more_asserts::assert_gt;

  //   let contents = read_input(INPUT_FILENAME).unwrap();
  //   let result = part_2(&contents);
  //   assert_gt!(result, 21409);
  //   assert_eq!(result, 9064949303801);
  // }
}
