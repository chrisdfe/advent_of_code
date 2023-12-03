pub fn run(contents: &str) -> u32 {
  0
}

#[cfg(test)]
mod tests {
  #[test]
  pub fn day_3_part_1_example_works() {
    let contents =
      crate::utils::read_input(super::super::constants::PART_1_EXAMPLE_INPUT_FILENAME).unwrap();
    let result = super::run(&contents);
    assert_eq!(result, 4361);
  }
}
