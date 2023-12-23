const INPUT_FILENAME: &str = "./src/day_9/input.txt";

fn tuple_vec_from_vec(input: &Vec<i32>) -> Vec<(i32, i32)> {
  input
    .iter()
    .enumerate()
    .fold(Vec::new(), |mut acc, (index, number)| {
      if index < input.len() - 1 {
        acc.push((*number, input[index + 1]));
      }

      acc
    })
}

fn find_next_value(numbers: &Vec<i32>) -> i32 {
  let differences = tuple_vec_from_vec(&numbers)
    .into_iter()
    .map(|(a, b)| b - a)
    .collect::<Vec<_>>();

  let differences_are_zero = differences.iter().find(|num| **num != 0) == None;

  let next_below_value = if differences_are_zero {
    0
  } else {
    find_next_value(&differences)
  };

  let result = numbers.last().unwrap() + next_below_value;
  println!("result: {}", result);
  result
}

fn part_1(input: &str) -> i32 {
  input
    .split('\n')
    .map(|line| {
      line
        .split(' ')
        .map(|num_string| num_string.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
    })
    .map(|numbers| find_next_value(&numbers))
    .reduce(|acc, result| acc + result)
    .unwrap()
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
  use crate::utils::read_input;

  use super::{part_1, INPUT_FILENAME};

  const EXAMPLE_INPUT_FILENAME: &str = "./src/day_9/example_input.txt";

  #[test]
  pub fn day_9_part_1_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 114);
  }

  // #[test]
  // pub fn day_9_part_1_solution_works() {
  //   let contents = read_input(INPUT_FILENAME).unwrap();
  //   let result = part_1(&contents);
  //   assert_eq!(result, 12737);
  // }

  // #[test]
  // pub fn day_9_part_2_example_works() {
  //   let contents = read_input(PART_2_EXAMPLE_INPUT_FILENAME).unwrap();
  //   let result = part_2(&contents);
  //   assert_eq!(result, 6);
  // }

  // #[test]
  // pub fn day_9_part_2_solution_works() {
  //   use more_asserts::assert_gt;

  //   let contents = read_input(INPUT_FILENAME).unwrap();
  //   let result = part_2(&contents);
  //   assert_gt!(result, 21409);
  //   assert_eq!(result, 9064949303801);
  // }
}
