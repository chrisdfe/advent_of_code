use std::collections::HashMap;

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

// A group of 1+ with number values
#[derive(Debug)]
struct GridNumber<'a> {
  cells: Vec<&'a Cell>,
}

impl<'a> GridNumber<'a> {
  fn get_total(&self) -> u32 {
    self
      .cells
      .iter()
      // these cells are guaranteed to be digits at this point
      .fold(String::new(), |acc, cell| {
        acc + cell.value.to_string().as_str()
      })
      .parse::<u32>()
      .unwrap()
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

  fn get_gear_neighbors(&self, grid: &'a Grid) -> Vec<&Cell> {
    self
      .get_neighbors(grid)
      .into_iter()
      .filter(|neighbor| neighbor.is_gear())
      .collect::<Vec<_>>()
  }
}

impl<'a> PartialEq for GridNumber<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.cells == other.cells && self.get_total() == other.get_total()
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

  fn get_valid_neighbor_positions(&self, cell: &Cell) -> Vec<Position> {
    let max_position = self.get_max_position();

    cell
      .position
      .get_neighbor_positions()
      .into_iter()
      .filter(|position| {
        position.x >= 0
          && position.y >= 0
          && position.x <= max_position.x
          && position.y <= max_position.y
      })
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
            result.push(GridNumber { cells: cell_buffer })
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
      .fold(0, |acc, grid_number| acc + grid_number.get_total())
  }

  fn get_valid_grid_number_gear_ratio_sum(&self) -> u32 {
    let gridnumbers = self.get_grid_numbers();

    let gear_gridnumber_neighbors = self
      .cells
      .iter()
      // Get gear symbol cells
      .filter(|cell| cell.is_gear())
      // Find adjacent gridnumbers
      .fold(Vec::new(), |mut acc, cell| {
        let gridnumber_neighbors: Vec<&GridNumber<'_>> = self
          .get_cell_neighbors(cell)
          .iter()
          .filter(|cell| cell.is_digit())
          // map digit cells to gridnumber
          .map(|cell| {
            gridnumbers
              .iter()
              .find(|gridnumber| gridnumber.cells.contains(cell))
              .unwrap()
          })
          // de-dupe
          .fold(Vec::new(), |mut acc, gridnumber| {
            if !acc.contains(&gridnumber) {
              acc.push(gridnumber);
            }

            acc
          });

        acc.push(gridnumber_neighbors);

        acc
      });

    let total = gear_gridnumber_neighbors
      .iter()
      .filter(|neighbors| neighbors.len() == 2)
      .fold(0, |acc, neighbors| {
        acc
          + neighbors.iter().fold(0, |acc, neighbor| {
            let neighbor_total = neighbor.get_total();

            if acc == 0 {
              neighbor_total
            } else {
              acc * neighbor_total
            }
          })
      });

    total
  }
}

// too low: 560570
pub fn run(contents: &str) -> u32 {
  let grid = Grid::from_raw_contents(contents);

  grid.get_valid_grid_number_gear_ratio_sum()
}

#[cfg(test)]
mod tests {
  use crate::utils::read_input;

  #[test]
  pub fn day_3_part_2_example_works() {
    let contents = read_input(super::super::constants::PART_1_EXAMPLE_INPUT_FILENAME).unwrap();
    let result = super::run(&contents);
    assert_eq!(result, 467835);
  }

  #[test]
  pub fn day_3_part_2_solution_works() {
    let contents = read_input(super::super::constants::INPUT_FILENAME).unwrap();
    let result = super::run(&contents);
    assert_eq!(result, 91622824);
  }
}
