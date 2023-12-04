mod day_1;
mod day_2;
mod day_3;
mod utils;

fn main() -> Result<(), std::io::Error> {
  let args = std::env::args().collect::<Vec<_>>();

  if args.len() == 2 {
    match args[1].as_str() {
      "day_1" => day_1::run()?,
      "day_2" => day_2::run()?,
      "day_3" => day_3::run()?,
      value => {
        println!("{value} not recognized")
      }
    }
  } else {
    println!("Running all days");
    day_1::run()?;
    day_2::run()?;
    day_3::run()?;
  };

  Ok(())
}
