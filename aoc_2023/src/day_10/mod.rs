const INPUT_FILENAME: &str = "./src/day_10/input.txt";

fn part_1(input: &str) -> i32 {
  0
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 10");
  let input = crate::utils::read_input(INPUT_FILENAME)?;

  let part_1_total = part_1(&input);
  println!("part_1 total {}", part_1_total);

  // let part_2_total = part_2(&input);
  // println!("part_2 total {}", part_2_total);

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::utils::read_input;

  use super::part_1;

  const EXAMPLE_INPUT_FILENAME_1: &str = "./src/day_10/example_input_1.txt";
  const EXAMPLE_INPUT_FILENAME_2: &str = "./src/day_10/example_input_2.txt";

  #[test]
  pub fn day_10_part_1_example_1_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME_1).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 4);
  }

  #[test]
  pub fn day_10_part_1_example_1_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME_2).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 8);
  }

  // #[test]
  // pub fn day_10_part_1_solution_works() {
  //   let contents = read_input(INPUT_FILENAME).unwrap();
  //   let result = part_1(&contents);
  //   assert_eq!(result, 2043183816);
  // }

  // #[test]
  // pub fn day_10_part_2_example_works() {
  //   let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
  //   let result = part_2(&contents);
  //   assert_eq!(result, 2);
  // }

  // #[test]
  // pub fn day_10_part_2_solution_works() {
  //   let contents = read_input(INPUT_FILENAME).unwrap();
  //   let result = part_2(&contents);
  //   assert_eq!(result, 1118);
  // }
}
