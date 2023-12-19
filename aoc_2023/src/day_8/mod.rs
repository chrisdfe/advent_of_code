use std::collections::HashMap;

pub const INPUT_FILENAME: &str = "src/day_8/input.txt";

fn get_instructions_and_element_map_from_input(
  input: &str,
) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
  let lines = input.split('\n').collect::<Vec<&str>>();
  let (raw_instructions, raw_element_defs) = match &lines[..] {
    // ignore empty newline between instructions and defs with _
    [raw_instructions, _, raw_element_defs @ ..] => (raw_instructions, raw_element_defs.to_vec()),
    _ => panic!("input isn't in the right format :("),
  };

  let instructions = raw_instructions.chars().collect::<Vec<_>>();

  let element_map: HashMap<&str, (&str, &str)> =
    raw_element_defs
      .iter()
      .fold(HashMap::new(), |mut acc, line| {
        let (key, raw_lr_defs) = line.split_once(" = ").unwrap();
        let (left, right) = &raw_lr_defs[1..raw_lr_defs.len() - 1]
          .split_once(", ")
          .unwrap();

        acc.insert(key, (left, right));
        acc
      });

  (instructions, element_map)
}

const TARGET_KEY: &str = "ZZZ";

fn part_1(input: &str) -> u64 {
  let (instructions, element_map) = get_instructions_and_element_map_from_input(input);

  let mut instructions_iter = instructions.into_iter().cycle();
  let mut steps = 0;
  let mut current_key: &&str = &&"AAA";

  while **current_key != *TARGET_KEY {
    steps += 1;
    let (left, right) = element_map.get(current_key).unwrap();
    let instruction = instructions_iter.next().unwrap();
    current_key = match instruction {
      'L' => left,
      'R' => right,
      other => panic!("Unsupported instruction found: {}", other),
    };
    println!("current_key: {}", current_key);
    println!("instruction: {}", instruction);
  }

  steps
}

fn part_2(input: &str) -> u64 {
  0
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 7");
  let input = crate::utils::read_input(INPUT_FILENAME)?;

  let part_1_total = part_1(&input);
  println!("part_1 total {}", part_1_total);

  // let part_2_total = part_2(&contents);
  // println!("part_2 total {}", part_2_total);

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::utils::read_input;

  use super::{part_1, part_2, INPUT_FILENAME};

  const EXAMPLE_INPUT_FILENAME: &str = "./src/day_8/example_input.txt";

  #[test]
  pub fn day_8_part_1_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 6);
  }

  #[test]
  pub fn day_8_part_1_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 12737);
  }

  #[test]
  pub fn day_8_part_2_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 5905);
  }

  // #[test]
  // pub fn day_8_part_2_solution_works() {
  //   use more_asserts::assert_gt;

  //   let contents = read_input(INPUT_FILENAME).unwrap();
  //   let result = part_2(&contents);
  //   assert_gt!(result, 248673364);
  //   assert_eq!(result, 248747492);
  // }
}
