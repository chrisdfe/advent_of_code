use std::collections::HashMap;

use crate::utils::read_input;

const INPUT_FILENAME: &str = "./src/day_4/input.txt";

#[derive(Clone)]
struct Card<'a> {
  winning_numbers: Vec<&'a str>,
  numbers: Vec<&'a str>,
}

impl<'a> Card<'a> {
  fn from_input(line: &str) -> Card {
    let (_, number_groups) = line.split_once(":").unwrap();
    let (raw_winning_numbers, raw_numbers) = number_groups.split_once("|").unwrap();

    let winning_numbers = get_number_list_from_string(raw_winning_numbers);
    let numbers = get_number_list_from_string(raw_numbers);

    Card {
      winning_numbers,
      numbers,
    }
  }
}

fn get_number_list_from_string(input: &str) -> Vec<&str> {
  input
    .trim()
    .split(" ")
    .filter(|num| num.len() > 0)
    .collect::<Vec<_>>()
}

fn create_cards_from_input(input: &str) -> Vec<Card<'_>> {
  input
    .lines()
    .map(|line| Card::from_input(line))
    .collect::<Vec<_>>()
}

fn part_1(contents: &str) -> u32 {
  create_cards_from_input(contents)
    .iter()
    .map(|card| {
      card.numbers.iter().fold(0, |acc, number| {
        if card.winning_numbers.contains(number) {
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

fn part_2(contents: &str) -> u32 {
  let cards = create_cards_from_input(contents);

  let card_instances =
    cards
      .iter()
      .enumerate()
      .fold(vec![1; cards.len()], |mut acc, (index, card)| {
        let winning_number_count = card.numbers.iter().fold(0, |acc, number| {
          if card.winning_numbers.contains(number) {
            acc + 1
          } else {
            acc
          }
        });

        let current_card_count = acc.get(index).unwrap().clone();
        for card_number in index + 1..index + 1 + winning_number_count {
          acc[card_number] += current_card_count;
        }

        acc
      });

  card_instances
    .iter()
    .fold(0, |acc, number| acc + number)
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 4");
  println!("reading contents of {}", INPUT_FILENAME);
  let contents = read_input(INPUT_FILENAME)?;

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

  const EXAMPLE_INPUT_FILENAME: &str = "./src/day_4/example_input.txt";

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
}
