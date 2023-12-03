use std::io::Error;

use crate::utils::read_input;

pub const INPUT_FILENAME: &'static str = "./src/day_3/input.txt";
pub const EXAMPLE_INPUT_FILENAME: &'static str = "./src/day_3/example_input.txt";

const NEIGHBOR_RANGE: [i8; 3] = [-1, 0, 1];

#[derive(Debug, Clone)]
struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn zero() -> Position {
    Position { x: 0, y: 0 }
  }

  fn get_neighbor_positions(&self) -> Vec<Position> {
    let x_neighbors = NEIGHBOR_RANGE.clone();
    let y_neighbors = NEIGHBOR_RANGE.clone();

    x_neighbors
      .into_iter()
      .flat_map(|x| {
        y_neighbors.into_iter().map(move |y| Position {
          x: self.x + x as i32,
          y: self.y + y as i32,
        })
      })
      .filter(|pos| pos != self)
      .collect::<Vec<_>>()
  }
}

impl PartialEq for Position {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

#[derive(Debug)]
struct Cell {
  position: Position,
  value: char,
}

impl Cell {
  fn is_valid_symbol(&self) -> bool {
    self.value != '.' && !self.value.is_digit(10)
  }

  fn is_gear(&self) -> bool {
    self.value == '*'
  }

  fn is_digit(&self) -> bool {
    self.value.is_digit(10)
  }
}

impl PartialEq for Cell {
  fn eq(&self, other: &Self) -> bool {
    self.position == other.position && self.value == other.value
  }
}

// A group of 1+ cells with number values
#[derive(Debug)]
struct GridNumber<'a> {
  cells: Vec<&'a Cell>,
  total: u32,
}

impl<'a> GridNumber<'a> {
  fn new(cells: Vec<&Cell>) -> GridNumber<'_> {
    let total = cells
      .iter()
      // For the sake of brevity/speed I'll assume these cells have digit values
      .fold(String::new(), |acc, cell| {
        acc + cell.value.to_string().as_str()
      })
      .parse::<u32>()
      .unwrap();

    GridNumber { cells, total }
  }

  fn get_neighbors(&'a self, grid: &'a Grid) -> Vec<&Cell> {
    self
      .cells
      .iter()
      .fold(Vec::new(), |mut acc, cell| {
        let neighbors = grid.get_cell_neighbors(cell);

        for neighbor in neighbors {
          if !acc.contains(&neighbor) && !self.cells.contains(&neighbor) {
            acc.push(neighbor);
          }
        }
        acc
      })
  }

  fn get_valid_symbol_neighbors(&self, grid: &'a Grid) -> Vec<&Cell> {
    self
      .get_neighbors(grid)
      .into_iter()
      .filter(|neighbor| neighbor.is_valid_symbol())
      .collect::<Vec<_>>()
  }
}

impl<'a> PartialEq for GridNumber<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.cells == other.cells && self.total == other.total
  }
}

struct Grid {
  cells: Vec<Cell>,
}

impl Grid {
  pub fn from_raw_contents(contents: &str) -> Grid {
    let cells = contents
      .lines()
      .enumerate()
      .flat_map(move |(y, line)| {
        line
          .chars()
          .enumerate()
          .map(move |(x, value)| Cell {
            position: Position {
              x: x as i32,
              y: y as i32,
            },
            value,
          })
      })
      .collect::<Vec<_>>();

    Grid { cells }
  }

  fn get_max_position(&self) -> Position {
    self
      .cells
      .iter()
      .fold(Position::zero(), |mut acc, cell| {
        if cell.position.x > acc.x {
          acc.x = cell.position.x;
        }

        if cell.position.y > acc.y {
          acc.y = cell.position.y;
        }

        acc
      })
  }

  fn is_valid_cell_position(&self, position: &Position) -> bool {
    let max_position = self.get_max_position();
    position.x >= 0
      && position.y >= 0
      && position.x <= max_position.x
      && position.y <= max_position.y
  }

  fn get_valid_neighbor_positions(&self, cell: &Cell) -> Vec<Position> {
    cell
      .position
      .get_neighbor_positions()
      .into_iter()
      .filter(|position| self.is_valid_cell_position(position))
      .collect::<Vec<_>>()
  }

  // It is assumed that target_cell is a cell in this Grid
  fn get_cell_neighbors(&self, target_cell: &Cell) -> Vec<&Cell> {
    self
      .get_valid_neighbor_positions(&target_cell)
      .into_iter()
      .map(|position| {
        let result = self
          .cells
          .iter()
          .find(|cell| cell.position == position)
          // get_valid_neighbor_positions ensures there will be a neighbor at that position
          // so unwrapping here is ok
          .unwrap();

        result
      })
      .collect::<Vec<_>>()
  }

  fn get_cells_grouped_by_line(&self) -> Vec<Vec<&Cell>> {
    self
      .cells
      .iter()
      .fold(Vec::new(), |mut acc, cell| {
        let y_idx = cell.position.y as usize;
        if y_idx as isize > acc.len() as isize - 1 {
          acc.push(Vec::new())
        }

        acc[y_idx].push(cell);
        acc
      })
  }

  fn get_grid_numbers(&self) -> Vec<GridNumber> {
    let mut result = Vec::new();
    let mut cell_buffer: Vec<&Cell> = Vec::new();

    for line in self.get_cells_grouped_by_line() {
      for cell in line {
        if cell.value.is_digit(10) {
          cell_buffer.push(cell);
        } else {
          if cell_buffer.len() > 0 {
            result.push(GridNumber::new(cell_buffer))
          }

          cell_buffer = Vec::new();
        }
      }
    }

    result
  }

  fn get_valid_grid_numbers(&self) -> Vec<GridNumber> {
    self
      .get_grid_numbers()
      .into_iter()
      .filter(|n| n.get_valid_symbol_neighbors(self).len() > 0)
      .collect::<Vec<_>>()
  }

  fn get_valid_grid_number_sum(&self) -> u32 {
    self
      .get_valid_grid_numbers()
      .into_iter()
      .fold(0, |acc, grid_number| acc + grid_number.total)
  }

  fn get_valid_grid_number_gear_ratio_sum(&self) -> u32 {
    let gridnumbers = self.get_grid_numbers();

    self
      .cells
      .iter()
      // Get all gear symbol cells
      .filter(|cell| cell.is_gear())
      // Find adjacent gridnumbers to cell
      .map(|gear_cell| {
        self
          // Find digit cell neighbors first
          .get_cell_neighbors(gear_cell)
          .iter()
          .filter(|cell| cell.is_digit())
          // then map digit cells to gridnumber
          .map(|cell| {
            gridnumbers
              .iter()
              .find(|gridnumber| gridnumber.cells.contains(cell))
              .unwrap()
          })
          // de-dupe
          .fold(Vec::new(), |mut neighbors, gridnumber| {
            if !neighbors.contains(&gridnumber) {
              neighbors.push(gridnumber);
            }

            neighbors
          })
      })
      // Gears without exactly 2 gridnumber neighbors are invalid
      .filter(|neighbors| neighbors.len() == 2)
      // Add total
      .fold(0, |acc, neighbors| {
        let neighbors_sum = neighbors.iter().fold(0, |acc, neighbor| {
          let neighbor_total = neighbor.total;

          if acc == 0 {
            neighbor_total
          } else {
            acc * neighbor_total
          }
        });

        acc + neighbors_sum
      })
  }
}

// too low: 560570
pub fn part_1(contents: &str) -> u32 {
  let grid = Grid::from_raw_contents(contents);

  grid.get_valid_grid_number_sum()
}

pub fn part_2(contents: &str) -> u32 {
  let grid = Grid::from_raw_contents(contents);

  grid.get_valid_grid_number_gear_ratio_sum()
}

pub fn run() -> Result<(), Error> {
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
  use super::{part_1, part_2, EXAMPLE_INPUT_FILENAME, INPUT_FILENAME};
  use crate::utils::read_input;

  #[test]
  pub fn day_3_part_1_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert_eq!(result, 4361);
  }

  #[test]
  pub fn day_3_part_1_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_1(&contents);
    assert!(result > 560570);
    assert_eq!(result, 560670);
  }

  #[test]
  pub fn day_3_part_2_example_works() {
    let contents = read_input(EXAMPLE_INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 467835);
  }

  #[test]
  pub fn day_3_part_2_solution_works() {
    let contents = read_input(INPUT_FILENAME).unwrap();
    let result = part_2(&contents);
    assert_eq!(result, 91622824);
  }
}
