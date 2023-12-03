use std::collections::HashMap;

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

fn process_line(line: &str) -> u32 {
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

// too high: 56175
// wrong: 56138
// wrong: 55255
// wrong: 43684
// wrong: 53222
// correct: 55260
pub fn run(contents: &str) -> u32 {
  contents
    .lines()
    .into_iter()
    .fold(0, |acc, line| acc + process_line(&line))
}
