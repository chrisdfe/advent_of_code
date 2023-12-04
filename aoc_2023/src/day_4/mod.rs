use crate::utils::read_input;

const EXAMPLE_INPUT_FILENAME: &str = "./src/day_4/example_input.txt";
const INPUT_FILENAME: &str = "./src/day_4/input.txt";

pub fn part_1(contents: &str) -> u32 {
  contents
    .lines()
    .map(|line| {
      let (_, number_groups) = line.split_once(":").unwrap();
      let (raw_winning_numbers, raw_my_numbers) = number_groups.split_once("|").unwrap();

      let winning_numbers = raw_winning_numbers
        .trim()
        .split(" ")
        .filter(|num| num.len() > 0)
        .collect::<Vec<_>>();

      let my_numbers = raw_my_numbers
        .trim()
        .split(" ")
        .filter(|num| num.len() > 0)
        .collect::<Vec<_>>();

      my_numbers.iter().fold(0, |acc, number| {
        if winning_numbers.contains(number) {
          match acc {
            0 => 1,
            _ => acc * 2,
          }
        } else {
          acc
        }
      })
    })
    .fold(0, |acc, card_points| acc + card_points)
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 4");
  println!("reading contents of {}", INPUT_FILENAME);
  let contents = read_input(INPUT_FILENAME)?;

  let part_1_total = part_1(&contents);
  println!("part_1 total {}", part_1_total);

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

  #[test]
  pub fn day_4_part_1_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 18519);
  }
}
