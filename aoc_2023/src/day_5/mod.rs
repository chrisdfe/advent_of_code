const INPUT_FILENAME: &str = "./src/day_4/input.txt";

fn part_1(input: &str) -> u32 {
  0
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 5");
  // let contents = crate::utils::read_input(INPUT_FILENAME)?;

  // let part_1_total = part_1(&contents);
  // println!("part_1 total {}", part_1_total);

  // let part_2_total = part_2(&contents);
  // println!("part_2 total {}", part_2_total);

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::utils::read_input;

  use super::part_1;

  const EXAMPLE_INPUT_FILENAME: &str = "./src/day_5/example_input.txt";

  #[test]
  pub fn day_5_part_1_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 35);
  }

  /*
  #[test]
  pub fn day_4_part_1_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 18519);
  }

  #[test]
  pub fn day_4_part_2_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 30);
  }

  #[test]
  pub fn day_4_part_2_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 11787590);
  }
  */
}
