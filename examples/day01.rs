use std::fs;

fn solve(input_file: &str, part: usize) -> u32 {
  let content = fs::read_to_string(input_file).expect("Failed to read input file");
  let lines: Vec<&str> = content.lines().collect();

  match part {
    1 => part_one(&lines),
    2 => part_two(&lines),
    _ => panic!("Invalid part number"),
  }
}

fn part_one(lines: &[&str]) -> u32 {
  lines
    .iter()
    .map(|&line| extract_calibration_value(line))
    .sum()
}

fn part_two(lines: &[&str]) -> u32 {
  lines
    .iter()
    .map(|&line| extract_calibration_value_with_words(line))
    .sum()
}

fn extract_calibration_value(line: &str) -> u32 {
  let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
  let first = digits.first().unwrap().to_digit(10).unwrap();
  let last = digits.last().unwrap().to_digit(10).unwrap();
  first * 10 + last
}

fn extract_calibration_value_with_words(line: &str) -> u32 {
  let words = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  ];
  let mut digits = Vec::new();

  for (i, c) in line.char_indices() {
    if c.is_digit(10) {
      digits.push(c.to_digit(10).unwrap());
    } else {
      for (j, word) in words.iter().enumerate() {
        if line[i..].starts_with(word) {
          digits.push(j as u32 + 1);
          break;
        }
      }
    }
  }

  digits.first().unwrap() * 10 + digits.last().unwrap()
}

fn main() {
  dbg!(solve("input/d01_full.txt", 1));
  dbg!(solve("input/d01_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve("input/d01_simple1.txt", 1), 142);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d01_full.txt", 1), 54081);
  }

  #[test]
  fn test_overlapping_words() {
    assert_eq!(extract_calibration_value_with_words("oneight"), 18);
    assert_eq!(extract_calibration_value_with_words("threeightwo"), 32);
    assert_eq!(extract_calibration_value_with_words("fiveighthree"), 53);
  }

  #[test]
  fn test_part2_simple() {
    assert_eq!(solve("input/d01_simple2.txt", 2), 281);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d01_full.txt", 2), 54649);
  }
}
