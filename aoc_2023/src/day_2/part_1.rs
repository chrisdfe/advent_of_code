const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

#[derive(Debug)]
struct Handful {
  red_cubes: u32,
  green_cubes: u32,
  blue_cubes: u32,
}

impl Handful {
  pub fn from_summary_text(handful_summary: &str) -> Handful {
    let statements = handful_summary.split(", ");

    let mut red_cubes: u32 = 0;
    let mut green_cubes: u32 = 0;
    let mut blue_cubes: u32 = 0;

    for statement in statements {
      let (count_as_string, cube_color) = match statement.split(" ").collect::<Vec<_>>()[..] {
        [count_as_string, cube_color] => (count_as_string, cube_color),
        _ => ("0", ""),
      };

      let count = count_as_string.parse::<u32>().unwrap();

      match cube_color {
        "red" => {
          red_cubes = count;
        }
        "green" => {
          green_cubes = count;
        }
        "blue" => {
          blue_cubes = count;
        }
        _ => (),
      }
    }

    Handful {
      red_cubes,
      green_cubes,
      blue_cubes,
    }
  }

  pub fn is_valid(&self) -> bool {
    self.red_cubes <= MAX_RED_CUBES
      && self.green_cubes <= MAX_GREEN_CUBES
      && self.blue_cubes <= MAX_BLUE_CUBES
  }
}

#[derive(Debug)]
struct Game {
  id: u32,
  handfuls: Vec<Handful>,
}

impl Game {
  pub fn from_line(line: &str) -> Game {
    let (title, game_summary) = match line.split(":").collect::<Vec<_>>()[..] {
      [title, text] => (title, text),
      _ => ("", ""),
    };

    let id = match title.split(" ").collect::<Vec<_>>()[..] {
      [_, id_as_string] => id_as_string.parse::<u32>().unwrap(),
      _ => 0,
    };

    let handfuls = game_summary
      .trim()
      .split("; ")
      .map(|handful_summary| Handful::from_summary_text(handful_summary))
      .collect::<Vec<_>>();

    Game { id, handfuls }
  }

  pub fn is_valid(&self) -> bool {
    self
      .handfuls
      .iter()
      .find(|handful| !handful.is_valid())
      .is_none()
  }
}

pub fn run(contents: &str) -> u32 {
  let games = contents
    .lines()
    .map(|line| Game::from_line(line))
    .filter(|game| game.is_valid())
    .collect::<Vec<_>>();

  let total = games.iter().fold(0, |acc, game| acc + game.id);

  total
}