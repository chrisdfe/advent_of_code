mod day_1;
mod day_10;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod utils;

fn main() -> Result<(), std::io::Error> {
  let args = std::env::args().collect::<Vec<_>>();

  if args.len() == 2 {
    match args[1].as_str() {
      "day_1" => day_1::run()?,
      "day_2" => day_2::run()?,
      "day_3" => day_3::run()?,
      "day_4" => day_4::run()?,
      "day_5" => day_5::run()?,
      "day_6" => day_6::run()?,
      "day_7" => day_7::run()?,
      "day_8" => day_8::run()?,
      "day_9" => day_9::run()?,
      "day_10" => day_10::run()?,
      value => {
        println!("{value} not recognized")
      }
    }
  } else {
    println!("Running all days");
    day_1::run()?;
    day_2::run()?;
    day_3::run()?;
    day_4::run()?;
    day_5::run()?;
    day_6::run()?;
    day_7::run()?;
    day_8::run()?;
    day_9::run()?;
    day_10::run()?;
  };

  Ok(())
}
