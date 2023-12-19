use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;

pub const INPUT_FILENAME: &str = "src/day_7/input.txt";

const HAND_LENGTH: usize = 5;

type CardPointMap = HashMap<char, u32>;

lazy_static! {
  static ref CARD_POINT_MAP_PART_1: CardPointMap = HashMap::from([
    ('2', 2),
    ('3', 3),
    ('4', 4),
    ('5', 5),
    ('6', 6),
    ('7', 7),
    ('8', 8),
    ('9', 9),
    ('T', 10),
    ('J', 11),
    ('Q', 12),
    ('K', 13),
    ('A', 14),
  ]);
  static ref CARD_POINT_MAP_PART_2: CardPointMap = HashMap::from([
    ('J', 1),
    ('2', 2),
    ('3', 3),
    ('4', 4),
    ('5', 5),
    ('6', 6),
    ('7', 7),
    ('8', 8),
    ('9', 9),
    ('T', 10),
    ('Q', 12),
    ('K', 13),
    ('A', 14),
  ]);
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
  bid: u32,
  hand_type: HandType,
  cards: Vec<char>,
  substituted_cards: Vec<char>,
}

fn get_hand_type_from_cards(cards: &Vec<char>) -> HandType {
  let card_counts: HashMap<char, u32> = cards
    .iter()
    .fold(HashMap::new(), |mut acc, card_char| {
      let next_count = match acc.get(&card_char) {
        Some(count) => count + 1,
        None => 1,
      };
      acc.insert(*card_char, next_count);

      acc
    });

  let card_counts_indexed_by_count: HashMap<u32, Vec<char>> =
    card_counts
      .keys()
      .fold(HashMap::new(), |mut acc, card_char| {
        let card_char_count = card_counts.get(card_char).unwrap();

        match acc.get_mut(card_char_count) {
          Some(card_char_vec) => {
            card_char_vec.push(*card_char);
          }
          None => {
            acc.insert(*card_char_count, vec![*card_char]);
          }
        };

        acc
      });

  let card_counts_ordered = card_counts_indexed_by_count
    .keys()
    .flat_map(|key| {
      card_counts_indexed_by_count
        .get(key)
        .unwrap()
        .into_iter()
        .map(|_| *key)
        .collect_vec()
    })
    .sorted()
    .rev()
    .collect_vec();

  match card_counts_ordered[..] {
    [5, ..] => HandType::FiveOfAKind,
    [4, ..] => HandType::FourOfAKind,
    [3, 2, ..] => HandType::FullHouse,
    [3, ..] => HandType::ThreeOfAKind,
    [2, 2, ..] => HandType::TwoPair,
    [2, ..] => HandType::OnePair,
    _ => HandType::HighCard,
  }
}

fn hand_from_input_line_part_1(line: &str) -> Hand {
  let (raw_cards_string, raw_bid) = line.split_once(" ").unwrap();

  let bid = raw_bid.parse::<u32>().unwrap();

  let cards = raw_cards_string.chars().collect_vec();

  let hand_type = get_hand_type_from_cards(&cards);

  Hand {
    bid,
    cards,
    substituted_cards: vec![],
    hand_type,
  }
}

fn get_card_count_map(cards: &Vec<char>) -> HashMap<&char, u32> {
  cards
    .iter()
    .fold(HashMap::new(), |mut acc, card_char| {
      let next_count = match acc.get(&card_char) {
        Some(count) => count + 1,
        None => 1,
      };

      acc.insert(&card_char, next_count);

      acc
    })
}

fn hand_from_input_line_part_2(line: &str) -> Hand {
  let (raw_cards_string, raw_bid) = line.split_once(" ").unwrap();

  let bid = raw_bid.parse::<u32>().unwrap();

  let cards = raw_cards_string.chars().collect_vec();

  // without the joker's special logic factored in
  let base_card_counts = get_card_count_map(&cards);

  let base_card_counts_indexed_by_count: HashMap<u32, Vec<char>> =
    base_card_counts
      .keys()
      .fold(HashMap::new(), |mut acc, card_char| {
        if **card_char == 'J' {
          acc
        } else {
          let card_char_count = base_card_counts.get(card_char).unwrap();

          match acc.get_mut(card_char_count) {
            Some(card_char_vec) => {
              card_char_vec.push(**card_char);
            }
            None => {
              acc.insert(*card_char_count, vec![**card_char]);
            }
          };

          acc
        }
      });

  let card_to_substitute_joker_with = {
    if base_card_counts_indexed_by_count.len() == 0 {
      'A'
    } else {
      let highest_key: u32 = base_card_counts_indexed_by_count
        .keys()
        .fold(0, |acc, key| if *key > acc { *key } else { acc });

      let most_frequent_cards = base_card_counts_indexed_by_count
        .get(&highest_key)
        .unwrap();

      if most_frequent_cards.len() > 1 {
        // two pair - use the strongest
        most_frequent_cards
          .iter()
          .fold((0, 'x'), |acc, card| {
            let (acc_card_points, _) = acc;
            let points = CARD_POINT_MAP_PART_2.get(card).unwrap();
            if *points > acc_card_points {
              (*points, *card)
            } else {
              acc
            }
          })
          .1
      } else {
        most_frequent_cards[0]
      }
    }
  };

  let substituted_cards = {
    cards
      .clone()
      .into_iter()
      .map(|card_char| {
        if card_char == 'J' {
          card_to_substitute_joker_with
        } else {
          card_char
        }
      })
      .collect_vec()
  };

  let hand_type = get_hand_type_from_cards(&substituted_cards);

  // println!("cards: {:?}", cards);
  println!(
    "card_to_substitute_joker_with: {:?}",
    card_to_substitute_joker_with
  );
  // println!("substituted_cards: {:?}", substituted_cards);
  // println!("hand_type: {:?}\n", hand_type);

  Hand {
    bid,
    cards,
    substituted_cards,
    hand_type,
  }
}

fn group_hands_by_type<'a>(hands: &'a Vec<Hand>) -> HashMap<HandType, Vec<&'a Hand>> {
  hands
    .iter()
    .fold(HashMap::new(), |mut acc, hand| {
      match acc.get_mut(&hand.hand_type) {
        Some(current) => {
          current.push(&hand);
        }
        None => {
          acc.insert(hand.hand_type.clone(), vec![&hand]);
        }
      }

      acc
    })
}

fn sort_hands_by_point_value<'a>(
  hands: &'a Vec<&'a Hand>,
  card_point_map: &CardPointMap,
) -> Vec<&'a Hand> {
  let mut sorted = hands.clone();
  sorted.sort_by(|a, b| {
    use std::cmp::Ordering;

    // Go through both hands from beginning to end and see if one is higher
    for index in 0..HAND_LENGTH {
      let a_value = card_point_map.get(&a.cards[index]).unwrap();
      let b_value = card_point_map.get(&b.cards[index]).unwrap();

      match a_value.cmp(b_value) {
        // if they're the same just continue onto the next set of chars
        Ordering::Equal => (),
        // Otherwise the winner is the hand with the card with the greater value
        ordering => {
          return ordering;
        }
      }
    }

    Ordering::Equal
  });

  sorted
}

fn get_total_winnings(hands_sorted_by_rank: &Vec<&Hand>) -> u64 {
  hands_sorted_by_rank
    .into_iter()
    .enumerate()
    .map(|(index, hand)| {
      // rank = index + 1
      hand.bid as u64 * (index as u64 + 1)
    })
    // Add them all together
    .fold(0, |acc, winnings| acc + winnings)
}

fn part_1(input: &str) -> u64 {
  let hands = input
    .split('\n')
    .map(hand_from_input_line_part_1)
    .collect_vec();

  let hands_grouped_by_type = group_hands_by_type(&hands);

  let hands_sorted_by_rank = hands_grouped_by_type
    .keys()
    // Sort keys low to high
    .sorted()
    .into_iter()
    .map(|key| hands_grouped_by_type.get(key).unwrap())
    // Sort groups of hands using the second ordering rule from the prompt
    .flat_map(|hands| sort_hands_by_point_value(hands, &CARD_POINT_MAP_PART_1))
    .collect_vec();

  get_total_winnings(&hands_sorted_by_rank)
}

fn part_2(input: &str) -> u64 {
  let hands = input
    .split('\n')
    .map(hand_from_input_line_part_2)
    .collect_vec();

  let hands_grouped_by_type = group_hands_by_type(&hands);

  // println!("hands sorted by rank:");

  let hands_sorted_by_rank = hands_grouped_by_type
    .keys()
    // Sort keys low to high
    .sorted()
    .into_iter()
    .map(|key| hands_grouped_by_type.get(key).unwrap())
    // Sort groups of hands using the second ordering rule from the prompt
    // TODO - use the base card set, not the substituted card set
    .flat_map(|hands| sort_hands_by_point_value(hands, &CARD_POINT_MAP_PART_2))
    .collect_vec();

  for (index, hand) in hands_sorted_by_rank.iter().enumerate() {
    // println!("{}: {:?}", index + 1, hand);
  }

  get_total_winnings(&hands_sorted_by_rank)
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 7");
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

  const EXAMPLE_INPUT_FILENAME: &str = "./src/day_7/example_input.txt";

  #[test]
  pub fn day_7_part_1_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 6440);
  }

  #[test]
  pub fn day_7_part_1_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 247815719);
  }

  #[test]
  pub fn day_7_part_2_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 5905);
  }

  #[test]
  pub fn day_7_part_2_solution_works() {
    use more_asserts::assert_gt;

    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_gt!(result, 248673364);
    assert_eq!(result, 248747492);
  }
}
