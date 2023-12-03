const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_blue_CUBES: u32 = 14;

struct Handful {
  red_cubes: u32,
  green_cubes: u32,
  blue_cubes: u32,
}

struct Game {
  id: u32,
  handfuls: Vec<Handful>,
}

fn parse_line_as_game(line: &str) -> Game {
  Game {
    id: 0,
    handfuls: Vec::new(),
  }
}

pub fn run(contents: &str) -> usize {
  println!("day 2");
  println!("{}", contents);
  0
}
