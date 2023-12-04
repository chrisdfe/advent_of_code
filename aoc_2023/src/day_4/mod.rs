use std::collections::HashMap;

use crate::utils::read_input;

const INPUT_FILENAME: &str = "./src/day_4/input.txt";

fn get_number_list_from_string(input: &str) -> Vec<&str> {
  input
    .trim()
    .split(" ")
    .filter(|num| num.len() > 0)
    .collect::<Vec<_>>()
}

#[derive(Clone)]
struct Card<'a> {
  number: u32,
  winning_numbers: Vec<&'a str>,
  my_numbers: Vec<&'a str>,
}

impl<'a> Card<'a> {
  fn from_input(line: &str) -> Card {
    let (card_title, number_groups) = line.split_once(":").unwrap();
    let (_, raw_card_number) = card_title.split_once("Card ").unwrap();
    let number = raw_card_number.trim().parse::<u32>().unwrap();
    let (raw_winning_numbers, raw_my_numbers) = number_groups.split_once("|").unwrap();

    let winning_numbers = get_number_list_from_string(raw_winning_numbers);
    let my_numbers = get_number_list_from_string(raw_my_numbers);

    Card {
      number,
      winning_numbers,
      my_numbers,
    }
  }
}

fn create_cards_from_input(input: &str) -> Vec<Card<'_>> {
  input
    .lines()
    .map(|line| Card::from_input(line))
    .collect::<Vec<_>>()
}

// key = card number, value = instances of that card
type CardCountMap = HashMap<u32, u32>;

fn create_default_card_count_map<'a>(cards: &Vec<Card<'_>>) -> CardCountMap {
  cards
    .iter()
    .fold(HashMap::new(), |mut acc, card| {
      acc.insert(card.number, 1);
      acc
    })
}

fn part_1(contents: &str) -> u32 {
  create_cards_from_input(contents)
    .iter()
    .map(|card| {
      card.my_numbers.iter().fold(0, |acc, number| {
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
  let starting_map = create_default_card_count_map(&cards);

  let card_count_map: CardCountMap = cards.iter().fold(starting_map, |mut acc, card| {
    let winning_number_count = card.my_numbers.iter().fold(0, |acc, number| {
      if card.winning_numbers.contains(number) {
        acc + 1
      } else {
        acc
      }
    });

    let current_card_count = acc.get(&card.number).unwrap().clone();
    for card_number in card.number + 1..card.number + 1 + winning_number_count {
      // If the card number is out of bounds this will fail
      if let Some(current_count) = acc.get(&card_number) {
        acc
          .insert(card_number, current_count + current_card_count)
          .unwrap();
      }
    }

    acc
  });

  card_count_map
    .values()
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
