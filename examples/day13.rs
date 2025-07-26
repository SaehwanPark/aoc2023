use std::fs;

fn solve(input_file: &str, part: usize) -> usize {
  let contents = fs::read_to_string(input_file).expect("Failed to read input file");
  solve_string(&contents, part)
}

fn solve_string(input: &str, part: usize) -> usize {
  let patterns: Vec<Vec<Vec<char>>> = input
    .trim()
    .split("\n\n")
    .map(|pattern| {
      pattern
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
    })
    .collect();

  patterns
    .iter()
    .enumerate()
    .map(|(i, pattern)| {
      let summary = summarize_pattern(pattern, i + 1, part);
      summary
    })
    .sum()
}

fn summarize_pattern(pattern: &Vec<Vec<char>>, _pattern_number: usize, part: usize) -> usize {
  let rows = pattern.len();
  let cols = pattern[0].len();

  let mut result = 0;

  // check for vertical reflection
  for i in 1..cols {
    if is_reflection(pattern, false, i, part) {
      result += i;
    }
  }

  // check for horizontal reflection
  for i in 1..rows {
    if is_reflection(pattern, true, i, part) {
      result += i * 100;
    }
  }

  result
}

fn is_reflection(pattern: &Vec<Vec<char>>, horizontal: bool, line: usize, part: usize) -> bool {
  let (outer, inner) = if horizontal {
    (pattern.len(), pattern[0].len())
  } else {
    (pattern[0].len(), pattern.len())
  };

  let mut differences = 0;
  let max_reflect = line.min(outer - line);

  for i in 0..max_reflect {
    for j in 0..inner {
      let (a, b) = if horizontal {
        (pattern[line - i - 1][j], pattern[line + i][j])
      } else {
        (pattern[j][line - i - 1], pattern[j][line + i])
      };
      if a != b {
        differences += 1;
        if differences > part - 1 {
          return false;
        }
      }
    }
  }

  differences == part - 1 && max_reflect > 0
}

fn main() {
  dbg!(solve("input/d13_full.txt", 1));
  dbg!(solve("input/d13_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve("input/d13_simple.txt", 1), 405);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d13_full.txt", 1), 31739);
  }

  #[test]
  fn test_part2_simple() {
    assert_eq!(solve("input/d13_simple.txt", 2), 400);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d13_full.txt", 2), 31539);
  }
}
