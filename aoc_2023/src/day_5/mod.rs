use std::ops::{Range, RangeInclusive};

use itertools::Itertools;

const INPUT_FILENAME: &str = "./src/day_5/input.txt";

fn get_min_in_vec(vec: &Vec<i64>) -> i64 {
  vec
    .iter()
    .fold(i64::MAX, |acc, num| if *num < acc { *num } else { acc })
}

fn ranges_overlap(a: &Range<i64>, b: &Range<i64>) -> bool {
  false
}

// Taken from https://stackoverflow.com/questions/26998485/is-it-possible-to-print-a-number-formatted-with-thousand-separator-in-rust
fn format_number_with_commas(num: i64) -> String {
  num
    .to_string()
    .as_bytes()
    .rchunks(3)
    .rev()
    .map(std::str::from_utf8)
    .collect::<Result<Vec<&str>, _>>()
    .unwrap()
    .join(",")
}

/*
// Algorithm from
// https://stackoverflow.com/questions/3269434/whats-the-most-efficient-way-to-test-if-two-ranges-overlap
fn ranges_overlap(a: &Range<i64>, b: &Range<i64>) -> bool {
  (a.start >= b.start && a.start <= b.end)
    || (a.end >= b.start && a.end <= b.end)
    || (b.start >= a.start && b.start <= a.end)
    || (b.end >= a.start && b.end <= a.end)
}
*/

struct AlmanacMap {
  source_range: RangeInclusive<i64>,
  dest_range: RangeInclusive<i64>,
}

impl AlmanacMap {
  fn from_input(input: &str) -> AlmanacMap {
    let (dest_start, source_start, range_length) = match input.split(" ").collect::<Vec<_>>()[..] {
      [raw_dest_start, raw_source_start, raw_range_length] => (
        raw_dest_start.parse::<i64>().unwrap(),
        raw_source_start.parse::<i64>().unwrap(),
        raw_range_length.parse::<i64>().unwrap(),
      ),
      // This isn't going to happen
      _ => (0, 0, 0),
    };
    println!(
      "creating AlmanacMap: dest {} src {} range {}",
      dest_start, source_start, range_length
    );

    let source_range = source_start..=source_start + range_length;
    let dest_range = dest_start..=dest_start + range_length;

    AlmanacMap {
      source_range,
      dest_range,
    }
  }
}

fn create_almanac_map_from_input(input: String) -> Vec<AlmanacMap> {
  // println!("input: {}", input);
  let (_, mapping_def_lines) = input.split_once('\n').unwrap();

  println!("creating {} AlmanacMaps", mapping_def_lines.len());

  mapping_def_lines
    .split("\n")
    .map(AlmanacMap::from_input)
    .collect_vec()
}

fn part_1(input: &str) -> i64 {
  let chunks = input.split("\n\n").collect::<Vec<_>>();

  let (seed_defs, source_dest_map_defs) = match chunks.as_slice() {
    [seed_defs, source_dest_map_defs @ ..] => (seed_defs, source_dest_map_defs),
    _ => return 0,
  };

  let seeds = seed_defs
    .split_once("seeds: ")
    .unwrap()
    .1
    .split(' ')
    .map(|int_as_str| int_as_str.parse::<i64>().unwrap())
    .collect::<Vec<_>>();

  let result = source_dest_map_defs
    .into_iter()
    .map(|input| create_almanac_map_from_input(String::from(*input)))
    .fold(seeds, |acc, almanac_maps| {
      acc
        .iter()
        .map(|source_value| {
          for almanac_map in almanac_maps.iter() {
            let AlmanacMap {
              source_range,
              dest_range,
            } = &almanac_map;

            if source_range.contains(source_value) {
              let diff = source_value - source_range.start();
              return dest_range.start() + diff;
            }
          }

          *source_value
        })
        .collect_vec()
    });

  // return min value in array
  get_min_in_vec(&result)
}

fn part_2(input: &str) -> i64 {
  let chunks = input.split("\n\n").collect::<Vec<_>>();

  let (seed_ranges, almanac_map_categories) = match chunks.as_slice() {
    [seed_line, almanac_map_lines @ ..] => {
      let (_, raw_seed_numbers) = seed_line.split_once("seeds: ").unwrap();

      let seed_ranges = raw_seed_numbers
        .split(' ')
        .map(|int_as_str| int_as_str.parse::<i64>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|chunk| match chunk.collect::<Vec<_>>()[..] {
          [range_start, range_width] => range_start..=range_start + range_width,
          // this won't happen
          _ => 0..=0,
        })
        .collect_vec();

      let almanac_map_categories = almanac_map_lines
        .into_iter()
        .map(|input| create_almanac_map_from_input(String::from(*input)))
        .collect_vec();

      (seed_ranges, almanac_map_categories)
    }
    _ => return 0,
  };

  let seed_ranges =
    almanac_map_categories
      .into_iter()
      .flatten()
      .fold(seed_ranges, |acc, almanac_map| {
        acc
          .iter()
          .flat_map(|seed_range| {
            let AlmanacMap {
              source_range,
              dest_range,
            } = &almanac_map;

            println!("seed_range: {:?}", seed_range);
            println!("source_range: {:?}", source_range);
            let source_dest_diff = dest_range.start() - source_range.start();
            let dest_seed_range =
              seed_range.start() + source_dest_diff..=seed_range.end() + source_dest_diff;

            println!("dest_range: {:?}", dest_range);
            println!("diff: {:?}", source_dest_diff);
            // TODO - explain
            let result = if seed_range.start() >= source_range.start()
              && seed_range.end() <= source_range.end()
            {
              println!("inside: seed: {:?} source: {:?}", seed_range, source_range);
              // source completely covers seed_range
              vec![dest_seed_range]
            } else if seed_range.start() < source_range.start()
              && seed_range.end() > source_range.start()
              && seed_range.end() <= source_range.end()
            {
              println!(
                "starts before: seed: {:?} source: {:?}",
                seed_range, source_range
              );
              // seed range partially overlaps - starts before source
              // split into 2 ranges
              vec![
                *seed_range.start()..=*source_range.start() - 1,
                *dest_range.start()..=*dest_seed_range.end(),
              ]
            } else if seed_range.start() >= source_range.start()
              && seed_range.start() < source_range.end()
              && seed_range.end() > source_range.end()
            {
              println!(
                "starts after: seed: {:?} source: {:?}",
                seed_range, source_range
              );
              // seed range partially overlaps - starts after source
              // split into 2 ranges
              vec![
                *dest_seed_range.start()..=*dest_range.end(),
                *source_range.end() + 1..=*seed_range.end(),
              ]
            } else if seed_range.start() < source_range.start()
              && seed_range.end() > source_range.end()
            {
              println!(
                "starts before and after: seed: {:?} source: {:?}",
                seed_range, source_range
              );
              // seed range overlaps - starts before source and ends after source
              // split into 3 ranges
              vec![
                *seed_range.start()..=*source_range.start() - 1,
                dest_range.clone(),
                *source_range.end() + 1..=*seed_range.end(),
              ]
            } else {
              println!(
                "no overlap: seed: {:?} source: {:?}",
                seed_range, source_range
              );
              vec![seed_range.clone()]
            };
            println!("result: {:?}", result);
            println!("---");
            result
          })
          .collect_vec()
      });

  seed_ranges
    .into_iter()
    .fold(i64::MAX, |acc, seed_range| {
      std::cmp::min(acc, *seed_range.start())
    })
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 5");
  let contents = crate::utils::read_input(INPUT_FILENAME)?;

  // let part_1_total = part_1(&contents);
  // println!("part_1 total {}", part_1_total);

  let part_2_total = part_2(&contents);
  println!("part_2 total {}", part_2_total);

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::utils::read_input;

  use super::{part_1, part_2, INPUT_FILENAME};

  const EXAMPLE_INPUT_FILENAME: &str = "./src/day_5/example_input.txt";

  #[test]
  pub fn day_5_part_1_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 35);
  }

  #[test]
  pub fn day_5_part_1_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 379811651);
  }

  #[test]
  pub fn day_5_part_2_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 46);
  }

  /*
  #[test]
  pub fn day_4_part_2_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 11787590);
  }
  */
}
