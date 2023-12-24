use std::collections::VecDeque;

use lazy_static::lazy_static;

const INPUT_FILENAME: &str = "./src/day_10/input.txt";

lazy_static! {
  static ref VALID_NEXT_PIPES_FROM_STARTING_POINT: [(RelativeCoordinates, Vec<char>); 4] = [
    (RelativeCoordinates::north(), vec!['|', 'F', '7']),
    (RelativeCoordinates::east(), vec!['-', '7', 'J']),
    (RelativeCoordinates::south(), vec!['|', 'J', 'L']),
    (RelativeCoordinates::west(), vec!['-', 'F', 'L']),
  ];
}

#[derive(Debug, PartialEq, Clone)]
struct Coordinates {
  column: usize,
  row: usize,
}

impl Coordinates {
  fn from_index(column_count: &usize, index: &usize) -> Coordinates {
    let row = index / column_count;
    let column = index % column_count;

    Coordinates { row, column }
  }

  fn to_index(&self, column_count: &usize) -> usize {
    (self.row * column_count) + self.column
  }
}

#[derive(Debug, Clone)]
struct RelativeCoordinates {
  column: isize,
  row: isize,
}

impl RelativeCoordinates {
  fn north() -> RelativeCoordinates {
    RelativeCoordinates { column: 0, row: -1 }
  }

  fn south() -> RelativeCoordinates {
    RelativeCoordinates { column: 0, row: 1 }
  }

  fn east() -> RelativeCoordinates {
    RelativeCoordinates { column: 1, row: 0 }
  }

  fn west() -> RelativeCoordinates {
    RelativeCoordinates { column: -1, row: 0 }
  }
}

#[derive(Debug, PartialEq)]
struct PipePiece {
  coordinates: Coordinates,
  char: char,
}

#[derive(Debug)]
struct PipeMap {
  pipes: Vec<PipePiece>,
  column_count: usize,
  row_count: usize,
  starting_coordinates: Coordinates,
}

impl PipeMap {
  fn from_input(input: &str) -> PipeMap {
    let lines = input.split('\n').collect::<Vec<_>>();

    println!("lines");

    let pipes = lines
      .iter()
      .enumerate()
      .flat_map(|(row, line)| {
        line
          .chars()
          .enumerate()
          .into_iter()
          .map(move |(column, char)| {
            let coordinates = Coordinates { column, row };
            //
            PipePiece { coordinates, char }
          })
      })
      .collect::<Vec<_>>();

    let row_count = lines.len();
    let column_count = lines[0].len();

    let s_index = pipes
      .iter()
      .enumerate()
      .find_map(|(index, pipe)| if pipe.char == 'S' { Some(index) } else { None })
      .unwrap();

    let starting_coordinates = Coordinates::from_index(&column_count, &s_index);

    PipeMap {
      pipes,
      row_count,
      column_count,
      starting_coordinates,
    }
  }

  fn get_pipe_piece_at_coordinates(&self, coordinates: &Coordinates) -> &PipePiece {
    let index = coordinates.to_index(&self.column_count);
    &self.pipes[index]
  }

  fn build_pipe_loop_with_distances(&self) -> Vec<(u32, &PipePiece)> {
    let mut result: Vec<(u32, &PipePiece)> = Vec::new();
    let mut queue: VecDeque<(u32, &PipePiece)> = VecDeque::new();

    let starting_coordinates = self.starting_coordinates.clone();
    let starting_pipe_piece = self.get_pipe_piece_at_coordinates(&starting_coordinates);
    queue.push_back((0, starting_pipe_piece));

    while let Some((current_pipe_piece_distance, current_pipe_piece)) = queue.pop_front() {
      let next_relative_coords: Vec<RelativeCoordinates> = match current_pipe_piece.char {
        '|' => vec![RelativeCoordinates::north(), RelativeCoordinates::south()],
        '-' => vec![RelativeCoordinates::west(), RelativeCoordinates::east()],
        'L' => vec![RelativeCoordinates::north(), RelativeCoordinates::east()],
        'J' => vec![RelativeCoordinates::north(), RelativeCoordinates::west()],
        '7' => vec![RelativeCoordinates::west(), RelativeCoordinates::south()],
        'F' => vec![RelativeCoordinates::east(), RelativeCoordinates::south()],
        'S' => {
          // Make sure the pipes can actually connect properly
          VALID_NEXT_PIPES_FROM_STARTING_POINT
            .clone()
            .into_iter()
            .fold(Vec::new(), |mut acc, (relative_coords, valid_chars)| {
              if let Ok(coords) =
                self.try_add_relative_coordinates(&current_pipe_piece.coordinates, &relative_coords)
              {
                let pipe_piece_at_coords = self.get_pipe_piece_at_coordinates(&coords);

                if valid_chars.contains(&pipe_piece_at_coords.char) {
                  acc.push(relative_coords)
                }
              };

              acc
            })
        }
        // realistically will only be '.' - ground
        _ => Vec::new(),
      };

      let next_coordinates = next_relative_coords
        .into_iter()
        // attempt to add coordinates together
        .map(|relative_coordinates| {
          self.try_add_relative_coordinates(&current_pipe_piece.coordinates, &relative_coordinates)
        })
        .filter(|addition_result| addition_result.is_ok())
        .map(|addition_result| addition_result.unwrap())
        // Filter out next coordinates that have already been logged
        .filter(|coordinates| {
          let pipe_piece = self.get_pipe_piece_at_coordinates(&coordinates);

          result
            .iter()
            .find(|(_, other_pipe_piece)| other_pipe_piece.coordinates == pipe_piece.coordinates)
            .is_none()
        })
        .collect::<Vec<_>>();

      result.push((current_pipe_piece_distance, current_pipe_piece));

      for coordinates in next_coordinates.into_iter() {
        let next_pipe_piece = self.get_pipe_piece_at_coordinates(&coordinates);
        queue.push_back((current_pipe_piece_distance + 1, next_pipe_piece));
      }
    }

    result
  }

  fn try_add_relative_coordinates(
    &self,
    coordinates: &Coordinates,
    coordinates_to_add: &RelativeCoordinates,
  ) -> Result<Coordinates, ()> {
    let added = RelativeCoordinates {
      column: coordinates.column as isize + coordinates_to_add.column,
      row: coordinates.row as isize + coordinates_to_add.row,
    };

    // Make sure the coordinates are within the bounds of the grid
    if added.row >= 0
      && added.row <= (self.row_count - 1) as isize
      && added.column >= 0
      && added.column <= (self.column_count - 1) as isize
    {
      Ok(Coordinates {
        column: added.column as usize,
        row: added.row as usize,
      })
    } else {
      Err(())
    }
  }
}

fn part_1(input: &str) -> u32 {
  let pipe_map = PipeMap::from_input(input);

  let loop_with_distances = pipe_map.build_pipe_loop_with_distances();
  println!("pipe_loop");
  for (distance, pipe) in loop_with_distances.iter() {
    println!("distance: {}, pipe: {:?}", distance, pipe);
  }

  loop_with_distances
    .into_iter()
    .fold(0, |acc, (distance, _)| std::cmp::max(acc, distance))
}

pub fn run() -> Result<(), std::io::Error> {
  println!("running day 10");
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

  const EXAMPLE_INPUT_1: &str = "./src/day_10/example_input_1.txt";
  const EXAMPLE_INPUT_2: &str = "./src/day_10/example_input_2.txt";

  #[test]
  pub fn day_10_part_1_example_1_works() {
    let contents = read_input(EXAMPLE_INPUT_1).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 4);
  }

  #[test]
  pub fn day_10_part_1_example_2_works() {
    let contents = read_input(EXAMPLE_INPUT_2).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 8);
  }

  #[test]
  pub fn day_10_part_1_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 6786);
  }

  // #[test]
  // pub fn day_10_part_2_example_works() {
  //   let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
  //   let result = part_2(&contents);
  //   assert_eq!(result, 2);
  // }

  // #[test]
  // pub fn day_10_part_2_solution_works() {
  //   let contents = read_input(INPUT_FILENAME).unwrap();
  //   let result = part_2(&contents);
  //   assert_eq!(result, 1118);
  // }
}
