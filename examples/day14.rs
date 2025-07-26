use std::collections::HashMap;
use std::fs;

fn solve(input_file: &str, part: usize) -> usize {
  let contents = fs::read_to_string(input_file).expect("Failed to read file");
  let mut grid: Vec<Vec<char>> = contents
    .lines()
    .map(|line| line.chars().collect())
    .collect();

  if part == 1 {
    tilt_north(&mut grid);
    calculate_load(&grid)
  } else {
    simulate_cycles(&mut grid, 1000000000)
  }
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
  let rows = grid.len();
  let cols = grid[0].len();

  for col in 0..cols {
    let mut write_pos = 0;
    for read_pos in 0..rows {
      match grid[read_pos][col] {
        'O' => {
          if read_pos != write_pos {
            grid[write_pos][col] = 'O';
            grid[read_pos][col] = '.';
          }
          write_pos += 1;
        }
        '#' => {
          write_pos = read_pos + 1;
        }
        _ => {}
      }
    }
  }
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
  let rows = grid.len();
  let cols = grid[0].len();

  for col in 0..cols {
    let mut write_pos = rows - 1;
    for read_pos in (0..rows).rev() {
      match grid[read_pos][col] {
        'O' => {
          if read_pos != write_pos {
            grid[write_pos][col] = 'O';
            grid[read_pos][col] = '.';
          }
          write_pos = write_pos.saturating_sub(1);
        }
        '#' => {
          write_pos = read_pos.saturating_sub(1);
        }
        _ => {}
      }
    }
  }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
  for row in grid.iter_mut() {
    let mut write_pos = 0;
    for read_pos in 0..row.len() {
      match row[read_pos] {
        'O' => {
          if read_pos != write_pos {
            row[write_pos] = 'O';
            row[read_pos] = '.';
          }
          write_pos += 1;
        }
        '#' => {
          write_pos = read_pos + 1;
        }
        _ => {}
      }
    }
  }
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
  for row in grid.iter_mut() {
    let mut write_pos = row.len() - 1;
    for read_pos in (0..row.len()).rev() {
      match row[read_pos] {
        'O' => {
          if read_pos != write_pos {
            row[write_pos] = 'O';
            row[read_pos] = '.';
          }
          write_pos = write_pos.saturating_sub(1);
        }
        '#' => {
          write_pos = read_pos.saturating_sub(1);
        }
        _ => {}
      }
    }
  }
}

fn calculate_load(grid: &Vec<Vec<char>>) -> usize {
  let rows = grid.len();
  grid
    .iter()
    .enumerate()
    .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (rows - i))
    .sum()
}

fn simulate_cycles(grid: &mut Vec<Vec<char>>, total_cycles: usize) -> usize {
  let mut seen = HashMap::new();
  let mut cycle = 0;

  while cycle < total_cycles {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);

    cycle += 1;

    let state = grid
      .iter()
      .map(|row| row.iter().collect::<String>())
      .collect::<Vec<String>>()
      .join("\n");
    if let Some(prev_cycle) = seen.get(&state) {
      let cycle_length = cycle - prev_cycle;
      let remaining = (total_cycles - cycle) % cycle_length;
      cycle = total_cycles - remaining;
    } else {
      seen.insert(state, cycle);
    }
  }

  calculate_load(grid)
}

fn main() {
  dbg!(solve("input/d14_full.txt", 1));
  dbg!(solve("input/d14_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve("input/d14_simple.txt", 1), 136);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d14_full.txt", 1), 107951);
  }

  #[test]
  fn test_part2_simple() {
    assert_eq!(solve("input/d14_simple.txt", 2), 64);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d14_full.txt", 2), 95736);
  }
}
