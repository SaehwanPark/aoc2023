use std::fs::File;
use std::io::{self, BufRead};

fn solve(path: &str, part: u8) -> io::Result<i64> {
  let file = File::open(&path)?;
  let reader = io::BufReader::new(file);

  let mut sum = 0;

  for line in reader.lines() {
    let numbers: Vec<i64> = line?
      .split_whitespace()
      .map(|s| s.parse().unwrap())
      .collect();

    sum += match part {
      1 => extrapolate_next_value(&numbers),
      2 => extrapolate_previous_value(&numbers),
      _ => panic!("Invalid part number"),
    };
  }

  Ok(sum)
}

fn extrapolate_next_value(sequence: &[i64]) -> i64 {
  if sequence.iter().all(|&x| x == 0) {
    return 0;
  }

  let differences: Vec<i64> = sequence.windows(2).map(|w| w[1] - w[0]).collect();

  sequence.last().unwrap() + extrapolate_next_value(&differences)
}

fn extrapolate_previous_value(sequence: &[i64]) -> i64 {
  if sequence.iter().all(|&x| x == 0) {
    return 0;
  }

  let differences: Vec<i64> = sequence.windows(2).map(|w| w[1] - w[0]).collect();

  sequence.first().unwrap() - extrapolate_previous_value(&differences)
}

fn main() {
  dbg!(solve("input/d09_full.txt", 1).unwrap());
  dbg!(solve("input/d09_full.txt", 2).unwrap());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_extrapolate_next_value() {
    assert_eq!(extrapolate_next_value(&[0, 3, 6, 9, 12, 15]), 18);
    assert_eq!(extrapolate_next_value(&[1, 3, 6, 10, 15, 21]), 28);
    assert_eq!(extrapolate_next_value(&[10, 13, 16, 21, 30, 45]), 68);
  }

  #[test]
  fn test_extrapolate_previous_value() {
    assert_eq!(extrapolate_previous_value(&[10, 13, 16, 21, 30, 45]), 5);
    assert_eq!(extrapolate_previous_value(&[0, 3, 6, 9, 12, 15]), -3);
    assert_eq!(extrapolate_previous_value(&[1, 3, 6, 10, 15, 21]), 0);
  }

  #[test]
  fn test_solve_simple_part1() {
    assert_eq!(solve("input/d09_simple.txt", 1).unwrap(), 114);
  }

  #[test]
  fn test_solve_full_part1() {
    assert_eq!(solve("input/d09_full.txt", 1).unwrap(), 1696140818);
  }

  #[test]
  fn test_solve_simple_part2() {
    assert_eq!(solve("input/d09_simple.txt", 2).unwrap(), 2);
  }

  // Uncomment and update the expected value when you have the result for Part 2
  #[test]
  fn test_solve_full_part2() {
    assert_eq!(solve("input/d09_full.txt", 2).unwrap(), 1152);
  }
}
