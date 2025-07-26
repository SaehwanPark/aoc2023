use std::fs;

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
  let lines: Vec<&str> = input.lines().collect();
  let times = lines[0]
    .split_whitespace()
    .skip(1)
    .map(|s| s.parse().unwrap())
    .collect();
  let distances = lines[1]
    .split_whitespace()
    .skip(1)
    .map(|s| s.parse().unwrap())
    .collect();
  (times, distances)
}

fn count_ways_to_win(time: u64, distance: u64) -> u64 {
  (1..time)
    .filter(|&hold_time| hold_time * (time - hold_time) > distance)
    .count() as u64
}

fn solve_part1(times: &[u64], distances: &[u64]) -> u64 {
  times
    .iter()
    .zip(distances.iter())
    .map(|(&t, &d)| count_ways_to_win(t, d))
    .product()
}

fn parse_input_part2(input: &str) -> (u64, u64) {
  let lines: Vec<&str> = input.lines().collect();
  let time = lines[0]
    .split(':')
    .nth(1)
    .unwrap()
    .replace(" ", "")
    .parse()
    .unwrap();
  let distance = lines[1]
    .split(':')
    .nth(1)
    .unwrap()
    .replace(" ", "")
    .parse()
    .unwrap();
  (time, distance)
}

pub fn solve(input_file: &str, part: usize) -> u64 {
  let contents = fs::read_to_string(input_file).expect("Failed to read input file");
  match part {
    1 => {
      let (times, distances) = parse_input(&contents);
      solve_part1(&times, &distances)
    }
    2 => {
      let (time, distance) = parse_input_part2(&contents);
      count_ways_to_win(time, distance)
    }
    _ => panic!("Invalid part number"),
  }
}

fn main() {
  dbg!(solve("input/d06_full.txt", 1));
  dbg!(solve("input/d06_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve("input/d06_simple.txt", 1), 288);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d06_full.txt", 1), 771628);
  }

  #[test]
  fn test_part2_simple() {
    assert_eq!(solve("input/d06_simple.txt", 2), 71503);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d06_full.txt", 2), 27363861);
  }
}
