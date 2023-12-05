use std::collections::HashMap;

const INPUT_FILENAME: &str = "./src/day_1/input.txt";

fn get_digit_word_map() -> HashMap<&'static str, usize> {
  HashMap::from([
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
  ])
}

fn part_1_process_line(line: &str) -> u32 {
  let numbers: Vec<u32> = line
    .chars()
    .into_iter()
    .fold(Vec::new(), |mut acc, char| {
      match char.to_string().parse::<u32>() {
        Ok(char_string_as_uint) => {
          acc.push(char_string_as_uint);
          acc
        }
        Err(_) => acc,
      }
    });

  let first_and_last = match numbers.len() {
    n if n >= 2 => {
      let first = *numbers.first().unwrap();
      let last = *numbers.last().unwrap();

      Some((first, last))
    }
    1 => Some((numbers[0], numbers[0])),
    _ => None,
  };

  if let Some((first, last)) = first_and_last {
    let result_as_string = format!("{}{}", first, last);

    // I can probably assume unwrapping is okay here
    result_as_string.parse::<u32>().unwrap()
  } else {
    0
  }
}

fn parse_slice_as_uint(slice: &str) -> Result<usize, ()> {
  let slice_as_string = slice.to_string();
  // Attempt to parse first char as digit
  let first_char_string = slice_as_string
    .chars()
    .nth(0)
    .unwrap()
    .to_string();

  match first_char_string.parse::<usize>() {
    Ok(char_string_as_uint) => Ok(char_string_as_uint),
    Err(_) => Err(()),
  }
}

fn finding_matching_digit_word_in_slice(slice: &str) -> Option<usize> {
  let digit_map = get_digit_word_map();

  let matching_key = digit_map.keys().find(|key| slice == **key);
  if let Some(key) = matching_key {
    return Some(*digit_map.get(key).unwrap());
  } else {
    None
  }
}

fn find_digit_in_slices(slices: Vec<&str>) -> Option<usize> {
  for slice in slices {
    if let Ok(digit) = parse_slice_as_uint(slice) {
      return Some(digit);
    } else {
      if let Some(digit) = finding_matching_digit_word_in_slice(slice) {
        return Some(digit);
      }
    }
  }

  None
}

fn part_2_process_line(line: &str) -> u32 {
  // Search for first number
  let first = {
    let forward_slices = (0..line.len())
      .flat_map(|first_index| {
        (first_index + 1..line.len() + 1).map(move |last_index| (first_index, last_index))
      })
      .map(|(first_index, last_index)| &line[first_index..last_index])
      .collect::<Vec<_>>();

    find_digit_in_slices(forward_slices)
  };

  // Search for the last number
  let last = {
    let backward_slices = (0..line.len())
      .rev()
      .flat_map(|first_index| {
        (first_index + 1..line.len() + 1)
          .rev()
          .map(|last_index| (first_index, last_index))
          .collect::<Vec<_>>()
      })
      .map(|(first, last)| &line[first..last])
      .collect::<Vec<_>>();

    find_digit_in_slices(backward_slices)
  };

  // Combine first + last
  if first != None && last != None {
    let result_as_string = format!("{}{}", first.unwrap(), last.unwrap());

    // I can probably assume unwrapping is okay here
    result_as_string.parse::<u32>().unwrap()
  } else {
    0
  }
}

fn part_1(contents: &str) -> u32 {
  contents
    .lines()
    .into_iter()
    .fold(0, |acc, line| acc + part_1_process_line(&line))
}

fn part_2(contents: &str) -> u32 {
  contents
    .lines()
    .into_iter()
    .fold(0, |acc, line| acc + part_2_process_line(&line))
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 1");
  println!("reading contents of {}", INPUT_FILENAME);
  let contents = crate::utils::read_input(INPUT_FILENAME)?;

  let result = part_1(&contents);
  println!("part one result: {}", result);

  let result = part_2(&contents);
  println!("part two result: {}", result);

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::utils::read_input;

  use super::{part_1, part_2, INPUT_FILENAME};

  const PART_1_EXAMPLE_INPUT_FILENAME: &str = "./src/day_1/part_1_example_input.txt";
  const PART_2_EXAMPLE_INPUT_FILENAME: &str = "./src/day_1/part_2_example_input.txt";

  #[test]
  pub fn day_1_part_1_example_works() {
    let contents = read_input(PART_1_EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 142);
  }

  #[test]
  pub fn day_1_part_1_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 55123);
  }

  #[test]
  pub fn day_1_part_2_example_works() {
    let contents = read_input(PART_2_EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 281);
  }

  #[test]
  pub fn day_1_part_2_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 55260);
  }
}
