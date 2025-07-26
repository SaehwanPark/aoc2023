use std::collections::HashSet;
use std::fs;

struct Schematic {
  grid: Vec<Vec<char>>,
}

impl Schematic {
  fn new(input: &str) -> Self {
    Self {
      grid: input.lines().map(|line| line.chars().collect()).collect(),
    }
  }

  fn dimensions(&self) -> (usize, usize) {
    (self.grid.len(), self.grid[0].len())
  }

  fn is_symbol(&self, row: usize, col: usize) -> bool {
    match self.grid.get(row).and_then(|r| r.get(col)) {
      Some(&ch) => !ch.is_ascii_digit() && ch != '.',
      None => false,
    }
  }

  fn extract_number(&self, row: usize, start_col: usize) -> (u32, usize) {
    let number: String = self.grid[row][start_col..]
      .iter()
      .take_while(|&&c| c.is_ascii_digit())
      .collect();

    (number.parse().unwrap(), number.len())
  }

  fn sum_part_numbers(&self) -> u32 {
    let (rows, cols) = self.dimensions();

    (0..rows)
      .flat_map(|row| {
        (0..cols).filter_map(move |col| {
          if self.grid[row][col].is_ascii_digit() {
            let (number, length) = self.extract_number(row, col);
            if self.is_adjacent_to_symbol(row, col, length) {
              Some(number)
            } else {
              None
            }
          } else {
            None
          }
        })
      })
      .sum()
  }

  fn is_adjacent_to_symbol(&self, row: usize, col: usize, length: usize) -> bool {
    let (rows, cols) = self.dimensions();
    let (row_start, row_end) = (row.saturating_sub(1), (row + 1).min(rows - 1));
    let (col_start, col_end) = (col.saturating_sub(1), (col + length).min(cols - 1));

    (row_start..=row_end).any(|r| (col_start..=col_end).any(|c| self.is_symbol(r, c)))
  }

  fn sum_gear_ratios(&self) -> u32 {
    let (rows, cols) = self.dimensions();

    (0..rows)
      .flat_map(|row| {
        (0..cols).filter_map(move |col| {
          if self.grid[row][col] == '*' {
            let adjacent_numbers = self.find_adjacent_numbers(row, col);
            if adjacent_numbers.len() == 2 {
              Some(adjacent_numbers[0] * adjacent_numbers[1])
            } else {
              None
            }
          } else {
            None
          }
        })
      })
      .sum()
  }

  fn find_adjacent_numbers(&self, row: usize, col: usize) -> Vec<u32> {
    let (rows, cols) = self.dimensions();
    let (row_start, row_end) = (row.saturating_sub(1), (row + 1).min(rows - 1));
    let (col_start, col_end) = (col.saturating_sub(1), (col + 1).min(cols - 1));

    let mut numbers = Vec::new();
    let mut visited = HashSet::new();

    for r in row_start..=row_end {
      let mut c = col_start;
      while c <= col_end {
        if self.grid[r][c].is_ascii_digit() && !visited.contains(&(r, c)) {
          let start_col = (0..=c)
            .rev()
            .find(|&i| !self.grid[r][i].is_ascii_digit())
            .map_or(0, |i| i + 1);
          let (number, length) = self.extract_number(r, start_col);
          numbers.push(number);
          visited.extend((start_col..start_col + length).map(|i| (r, i)));
          c = start_col + length;
        } else {
          c += 1;
        }
      }
    }
    numbers
  }
}

fn solve(input_file: &str, part: usize) -> u32 {
  let contents = fs::read_to_string(input_file).expect("Failed to read input file");
  let schematic = Schematic::new(&contents);

  match part {
    1 => schematic.sum_part_numbers(),
    2 => schematic.sum_gear_ratios(),
    _ => panic!("Invalid part number"),
  }
}

fn main() {
  dbg!(solve("input/d03_full.txt", 1));
  dbg!(solve("input/d03_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve("input/d03_simple.txt", 1), 4361);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d03_full.txt", 1), 521601);
  }

  #[test]
  fn test_rightmost_edge_cases() {
    let schematic = Schematic::new("123*\n...*\n456*");
    assert_eq!(schematic.sum_gear_ratios(), 56088);
  }

  #[test]
  fn test_part2_simple() {
    assert_eq!(solve("input/d03_simple.txt", 2), 467835);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d03_full.txt", 2), 80694070);
  }
}
