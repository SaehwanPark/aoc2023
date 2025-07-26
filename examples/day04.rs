use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve(input_file: &str, part: usize) -> u32 {
  let file = File::open(input_file).unwrap();
  let reader = BufReader::new(file);
  let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

  if part == 1 {
    solve_part1(&lines)
  } else {
    solve_part2(&lines)
  }
}

fn solve_part1(lines: &[String]) -> u32 {
  lines.iter().map(|line| calculate_card_points(line)).sum()
}

fn solve_part2(lines: &[String]) -> u32 {
  let mut card_counts: HashMap<usize, u32> = HashMap::new();

  for (i, line) in lines.iter().enumerate() {
    let card_number = i + 1;
    let current_count = *card_counts.entry(card_number).or_insert(1);
    let matches = count_matches(line);

    for j in 1..=matches {
      let next_card = card_number + j;
      if next_card <= lines.len() {
        *card_counts.entry(next_card).or_insert(1) += current_count;
      }
    }
  }

  card_counts.values().sum()
}

fn calculate_card_points(line: &str) -> u32 {
  let matches = count_matches(line);
  if matches == 0 {
    0
  } else {
    2u32.pow(matches as u32 - 1)
  }
}

fn count_matches(line: &str) -> usize {
  let parts: Vec<&str> = line.split(": ").nth(1).unwrap().split(" | ").collect();
  let winning_numbers: Vec<u32> = parts[0]
    .split_whitespace()
    .map(|n| n.parse().unwrap())
    .collect();
  let my_numbers: Vec<u32> = parts[1]
    .split_whitespace()
    .map(|n| n.parse().unwrap())
    .collect();

  my_numbers
    .iter()
    .filter(|&n| winning_numbers.contains(n))
    .count()
}

fn main() {
  dbg!(solve("input/d04_full.txt", 1));
  dbg!(solve("input/d04_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve("input/d04_simple.txt", 1), 13);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d04_full.txt", 1), 17782);
  }

  #[test]
  fn test_part2_simple() {
    assert_eq!(solve("input/d04_simple.txt", 2), 30);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d04_full.txt", 2), 8477787);
  }
}
