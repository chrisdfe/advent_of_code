fn process_line(line: &str) -> usize {
  let numbers: Vec<usize> = line
    .chars()
    .into_iter()
    .fold(Vec::new(), |mut acc, char| {
      match char.to_string().parse::<usize>() {
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
    result_as_string.parse::<usize>().unwrap()
  } else {
    0
  }
}

// part 1 too low: 36819
// part 1 too low: 37078
// part 1 correct: 55123
pub fn run(contents: &str) -> usize {
  contents
    .lines()
    .into_iter()
    .fold(0, |acc, line| acc + process_line(line))
}
