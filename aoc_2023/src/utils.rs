use std::fs::File;
use std::io::{prelude::*, Error};
use std::path::Path;

pub fn read_input(filename: &str) -> Result<String, Error> {
  // Create a path to the desired file
  let path = Path::new(filename);
  // let display = path.display();

  // Open the path in read-only mode, returns `io::Result<File>`
  let mut file = match File::open(&path) {
    Err(why) => {
      return Err(why);
    }
    Ok(file) => file,
  };

  // Read the file contents into a string, returns `io::Result<usize>`
  let mut contents = String::new();
  match file.read_to_string(&mut contents) {
    Err(why) => Err(why),
    Ok(_) => Ok(contents),
  }
}
