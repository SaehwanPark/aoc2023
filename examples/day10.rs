use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
  Vertical,
  Horizontal,
  NorthEast,
  NorthWest,
  SouthWest,
  SouthEast,
  Ground,
  Start,
}

impl Pipe {
  fn from_char(c: char) -> Self {
    match c {
      '|' => Pipe::Vertical,
      '-' => Pipe::Horizontal,
      'L' => Pipe::NorthEast,
      'J' => Pipe::NorthWest,
      '7' => Pipe::SouthWest,
      'F' => Pipe::SouthEast,
      '.' => Pipe::Ground,
      'S' => Pipe::Start,
      _ => panic!("Invalid pipe character"),
    }
  }
}

fn solve(input_file: &str, part: u32) -> usize {
  let grid = read_input(input_file);
  let (start_row, start_col) = find_start(&grid);
  let start_pipe = determine_start_pipe(&grid, start_row, start_col);

  let loop_tiles = find_loop(&grid, start_row, start_col, start_pipe);

  if part == 1 {
    loop_tiles.len() / 2
  } else {
    count_enclosed_tiles(&grid, &loop_tiles, start_row, start_col, start_pipe)
  }
}

fn read_input(filename: &str) -> Vec<Vec<Pipe>> {
  let file = File::open(filename).unwrap();
  let reader = io::BufReader::new(file);
  reader
    .lines()
    .map(|line| line.unwrap().chars().map(Pipe::from_char).collect())
    .collect()
}

fn find_start(grid: &[Vec<Pipe>]) -> (usize, usize) {
  for (row, line) in grid.iter().enumerate() {
    if let Some(col) = line.iter().position(|&p| p == Pipe::Start) {
      return (row, col);
    }
  }
  panic!("Start position not found");
}

fn determine_start_pipe(grid: &[Vec<Pipe>], row: usize, col: usize) -> Pipe {
  let north = row > 0
    && matches!(
      grid[row - 1][col],
      Pipe::Vertical | Pipe::SouthWest | Pipe::SouthEast
    );
  let south = row < grid.len() - 1
    && matches!(
      grid[row + 1][col],
      Pipe::Vertical | Pipe::NorthWest | Pipe::NorthEast
    );
  let west = col > 0
    && matches!(
      grid[row][col - 1],
      Pipe::Horizontal | Pipe::NorthEast | Pipe::SouthEast
    );
  let east = col < grid[0].len() - 1
    && matches!(
      grid[row][col + 1],
      Pipe::Horizontal | Pipe::NorthWest | Pipe::SouthWest
    );

  match (north, south, west, east) {
    (true, true, false, false) => Pipe::Vertical,
    (false, false, true, true) => Pipe::Horizontal,
    (true, false, true, false) => Pipe::NorthWest,
    (true, false, false, true) => Pipe::NorthEast,
    (false, true, true, false) => Pipe::SouthWest,
    (false, true, false, true) => Pipe::SouthEast,
    _ => panic!("Invalid start pipe configuration"),
  }
}

fn find_loop(
  grid: &[Vec<Pipe>],
  start_row: usize,
  start_col: usize,
  start_pipe: Pipe,
) -> HashSet<(usize, usize)> {
  let mut queue = VecDeque::new();
  let mut loop_tiles = HashSet::new();

  queue.push_back((start_row, start_col));
  loop_tiles.insert((start_row, start_col));

  while let Some((row, col)) = queue.pop_front() {
    let current_pipe = if (row, col) == (start_row, start_col) {
      start_pipe
    } else {
      grid[row][col]
    };
    let neighbors = get_neighbors(row, col, current_pipe);

    for (next_row, next_col) in neighbors {
      if next_row < grid.len()
        && next_col < grid[0].len()
        && !loop_tiles.contains(&(next_row, next_col))
      {
        if grid[next_row][next_col] != Pipe::Ground {
          queue.push_back((next_row, next_col));
          loop_tiles.insert((next_row, next_col));
        }
      }
    }
  }

  loop_tiles
}

fn get_neighbors(row: usize, col: usize, pipe: Pipe) -> Vec<(usize, usize)> {
  match pipe {
    Pipe::Vertical => vec![(row.wrapping_sub(1), col), (row + 1, col)],
    Pipe::Horizontal => vec![(row, col.wrapping_sub(1)), (row, col + 1)],
    Pipe::NorthEast => vec![(row.wrapping_sub(1), col), (row, col + 1)],
    Pipe::NorthWest => vec![(row.wrapping_sub(1), col), (row, col.wrapping_sub(1))],
    Pipe::SouthWest => vec![(row + 1, col), (row, col.wrapping_sub(1))],
    Pipe::SouthEast => vec![(row + 1, col), (row, col + 1)],
    Pipe::Ground | Pipe::Start => vec![],
  }
}

fn count_enclosed_tiles(
  grid: &[Vec<Pipe>],
  loop_tiles: &HashSet<(usize, usize)>,
  start_row: usize,
  start_col: usize,
  start_pipe: Pipe,
) -> usize {
  let mut count = 0;

  for row in 0..grid.len() {
    let mut inside = false;
    let mut last_bend = None;

    for col in 0..grid[0].len() {
      if loop_tiles.contains(&(row, col)) {
        let pipe = if (row, col) == (start_row, start_col) {
          start_pipe
        } else {
          grid[row][col]
        };
        match pipe {
          Pipe::Vertical => inside = !inside,
          Pipe::NorthEast | Pipe::SouthEast => last_bend = Some(pipe),
          Pipe::NorthWest | Pipe::SouthWest => {
            if let Some(last) = last_bend {
              if (last == Pipe::SouthEast && pipe == Pipe::NorthWest)
                || (last == Pipe::NorthEast && pipe == Pipe::SouthWest)
              {
                inside = !inside;
              }
            }
            last_bend = None;
          }
          _ => {}
        }
      } else if inside {
        count += 1;
      }
    }
  }

  count
}

fn main() {
  dbg!(solve("input/d10_full.txt", 1));
  dbg!(solve("input/d10_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple1() {
    assert_eq!(solve("input/d10_simple1.txt", 1), 4);
  }

  #[test]
  fn test_part1_simple2() {
    assert_eq!(solve("input/d10_simple2.txt", 1), 8);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d10_full.txt", 1), 6927);
  }

  #[test]
  fn test_part2_simple3() {
    assert_eq!(solve("input/d10_simple3.txt", 2), 4);
  }

  #[test]
  fn test_part2_simple4() {
    assert_eq!(solve("input/d10_simple4.txt", 2), 8);
  }

  #[test]
  fn test_part2_simple5() {
    assert_eq!(solve("input/d10_simple5.txt", 2), 10);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d10_full.txt", 2), 467);
  }
}
