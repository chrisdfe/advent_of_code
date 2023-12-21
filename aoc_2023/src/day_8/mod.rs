use std::collections::HashMap;

use itertools::Itertools;

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
  }

  steps
}

// Recursive function to return gcd of a and b
fn gcd(a: u64, b: u64) -> u64 {
  if a == 0 {
    b
  } else {
    gcd(b % a, a)
  }
}

// Function to return LCM of two numbers
fn lcm(a: u64, b: u64) -> u64 {
  (a / gcd(a, b)) * b
}

fn part_2(input: &str) -> u64 {
  let (instructions, element_map) = get_instructions_and_element_map_from_input(input);

  let element_map_keys = element_map.keys();

  let starting_keys = element_map_keys
    .filter(|key| key.ends_with('A'))
    .map(|key| *key)
    .collect::<Vec<_>>();

  println!("starting_keys: {:?}", starting_keys);

  let path_lengths = starting_keys.into_iter().map(|starting_key| {
    let mut length = 0;
    let mut instructions_iter = instructions.iter().cycle();
    let mut current_key = starting_key.clone();
    while !current_key.ends_with('Z') {
      let instruction = instructions_iter.next().unwrap();

      let (left, right) = element_map.get(current_key).unwrap();
      current_key = match instruction {
        'L' => &left,
        'R' => &right,
        other => panic!("Unsupported instruction found: {}", other),
      };

      length += 1;
    }

    length
  });

  // recursively calculate lcm
  path_lengths
    .reduce(|acc, length| lcm(acc, length))
    .unwrap()
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 7");
  let input = crate::utils::read_input(INPUT_FILENAME)?;

  let part_1_total = part_1(&input);
  println!("part_1 total {}", part_1_total);

  let part_2_total = part_2(&input);
  println!("part_2 total {}", part_2_total);

  Ok(())
}

#[cfg(test)]
mod tests {
  use more_asserts::assert_gt;

  use crate::utils::read_input;

  use super::{part_1, part_2, INPUT_FILENAME};

  const PART_1_EXAMPLE_INPUT_FILENAME: &str = "./src/day_8/part_1_example_input.txt";
  const PART_2_EXAMPLE_INPUT_FILENAME: &str = "./src/day_8/part_2_example_input.txt";

  #[test]
  pub fn day_8_part_1_example_works() {
    let contents = read_input(PART_1_EXAMPLE_INPUT_FILENAME).unwrap();
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
    let contents = read_input(PART_2_EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 6);
  }

  #[test]
  pub fn day_8_part_2_solution_works() {
    use more_asserts::assert_gt;

    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_gt!(result, 21409);
    assert_eq!(result, 9064949303801);
  }
}
