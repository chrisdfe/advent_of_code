pub const INPUT_FILENAME: &str = "src/day_6/input.txt";

type RaceTimeAndRecordDistance = (u64, u64);

fn strip_heading_from_line(line: &str) -> &str {
  let (_, rest) = line.split_once(':').unwrap();
  rest
}

fn split_line_by_arbitrary_amounts_of_whitespace(line: &str) -> Vec<String> {
  let mut result: Vec<String> = Vec::new();
  let mut buffer = String::new();

  let stripped_line = strip_heading_from_line(line);

  for char in String::from(stripped_line).chars() {
    if char == ' ' {
      if buffer.len() > 0 {
        result.push(buffer);
      }

      buffer = String::new();
    } else {
      buffer.push(char);
    }
  }

  if buffer.len() > 0 {
    result.push(buffer);
  }

  result
}

fn get_valid_combination_count((race_time, record_distance): RaceTimeAndRecordDistance) -> u64 {
  let distances_travelled = (1..race_time).map(|button_press_time| {
    let speed = button_press_time;
    let time_left = race_time - button_press_time;
    let distance_travelled = time_left * speed;
    distance_travelled
  });

  distances_travelled
    .into_iter()
    .filter(|distance| *distance > record_distance)
    .count() as u64
}

fn part_1(input: &str) -> u64 {
  let (race_times, record_distances) = {
    let mapped_lines = input
      .split('\n')
      .into_iter()
      .map(|line| {
        let number_strings = split_line_by_arbitrary_amounts_of_whitespace(line);
        number_strings
          .into_iter()
          .map(|number_string| number_string.parse::<u64>().unwrap())
      })
      .collect::<Vec<_>>();

    (mapped_lines[0].clone(), mapped_lines[1].clone())
  };

  let race_times_and_record_distances = race_times
    .into_iter()
    .zip(record_distances)
    .collect::<Vec<_>>();

  let record_breaker_counts = race_times_and_record_distances
    .into_iter()
    .map(|race_time_and_record_distance_tuple| {
      get_valid_combination_count(race_time_and_record_distance_tuple)
    })
    .collect::<Vec<_>>();

  let margin_of_error = record_breaker_counts
    .iter()
    .fold(1, |acc, num| acc * num);

  margin_of_error as u64
}

fn part_2(input: &str) -> u64 {
  let lines = input.split('\n').collect::<Vec<_>>();

  let race_time = {
    let number_string_vec = split_line_by_arbitrary_amounts_of_whitespace(lines[0]);
    let number_string = number_string_vec.join("");
    number_string.parse::<u64>().unwrap()
  };
  let distance_travelled = {
    let number_string_vec = split_line_by_arbitrary_amounts_of_whitespace(lines[1]);
    let number_string = number_string_vec.join("");
    number_string.parse::<u64>().unwrap()
  };

  let margin_of_error = get_valid_combination_count((race_time, distance_travelled));

  margin_of_error as u64
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 6");
  let contents = crate::utils::read_input(INPUT_FILENAME)?;

  let part_1_total = part_1(&contents);
  println!("part_1 total {}", part_1_total);

  let part_2_total = part_2(&contents);
  println!("part_2 total {}", part_2_total);

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::utils::read_input;

  use super::{part_1, part_2, INPUT_FILENAME};

  const EXAMPLE_INPUT_FILENAME: &str = "./src/day_6/example_input.txt";

  #[test]
  pub fn day_6_part_1_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 288);
  }

  #[test]
  pub fn day_6_part_1_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 1159152);
  }

  #[test]
  pub fn day_6_part_2_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 71503);
  }

  #[test]
  pub fn day_6_part_2_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 41513103);
  }
}
