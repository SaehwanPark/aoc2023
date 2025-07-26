use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
  x: usize,
  y: usize,
  direction: Direction,
}

fn solve(input_file: &str, part: usize) -> usize {
  let contents = fs::read_to_string(input_file).expect("Failed to read input file");
  let grid: Vec<Vec<char>> = contents
    .lines()
    .map(|line| line.chars().collect())
    .collect();

  if part == 1 {
    simulate_beam(
      &grid,
      Beam {
        x: 0,
        y: 0,
        direction: Direction::Right,
      },
    )
  } else {
    let mut max_energized = 0;
    let height = grid.len();
    let width = grid[0].len();

    // Top and bottom rows
    for x in 0..width {
      max_energized = max_energized.max(simulate_beam(
        &grid,
        Beam {
          x,
          y: 0,
          direction: Direction::Down,
        },
      ));
      max_energized = max_energized.max(simulate_beam(
        &grid,
        Beam {
          x,
          y: height - 1,
          direction: Direction::Up,
        },
      ));
    }

    // Left and right columns
    for y in 0..height {
      max_energized = max_energized.max(simulate_beam(
        &grid,
        Beam {
          x: 0,
          y,
          direction: Direction::Right,
        },
      ));
      max_energized = max_energized.max(simulate_beam(
        &grid,
        Beam {
          x: width - 1,
          y,
          direction: Direction::Left,
        },
      ));
    }

    max_energized
  }
}

fn simulate_beam(grid: &Vec<Vec<char>>, initial_beam: Beam) -> usize {
  let mut beams = vec![initial_beam];
  let mut energized = HashSet::new();
  let mut visited = HashSet::new();

  while let Some(beam) = beams.pop() {
    if !visited.insert((beam.x, beam.y, beam.direction)) {
      continue;
    }

    energized.insert((beam.x, beam.y));

    match grid[beam.y][beam.x] {
      '.' => beams.push(next_beam(beam, grid)),
      '/' => beams.push(reflect_forward_slash(beam, grid)),
      '\\' => beams.push(reflect_back_slash(beam, grid)),
      '|' => beams.extend(split_vertical(beam, grid)),
      '-' => beams.extend(split_horizontal(beam, grid)),
      _ => panic!("Invalid character in grid"),
    }
  }

  energized.len()
}

fn next_beam(beam: Beam, grid: &Vec<Vec<char>>) -> Beam {
  match beam.direction {
    Direction::Up if beam.y > 0 => Beam {
      x: beam.x,
      y: beam.y - 1,
      direction: Direction::Up,
    },
    Direction::Down if beam.y < grid.len() - 1 => Beam {
      x: beam.x,
      y: beam.y + 1,
      direction: Direction::Down,
    },
    Direction::Left if beam.x > 0 => Beam {
      x: beam.x - 1,
      y: beam.y,
      direction: Direction::Left,
    },
    Direction::Right if beam.x < grid[0].len() - 1 => Beam {
      x: beam.x + 1,
      y: beam.y,
      direction: Direction::Right,
    },
    _ => beam,
  }
}

fn reflect_forward_slash(beam: Beam, grid: &Vec<Vec<char>>) -> Beam {
  let new_direction = match beam.direction {
    Direction::Up => Direction::Right,
    Direction::Down => Direction::Left,
    Direction::Left => Direction::Down,
    Direction::Right => Direction::Up,
  };
  next_beam(
    Beam {
      x: beam.x,
      y: beam.y,
      direction: new_direction,
    },
    grid,
  )
}

fn reflect_back_slash(beam: Beam, grid: &Vec<Vec<char>>) -> Beam {
  let new_direction = match beam.direction {
    Direction::Up => Direction::Left,
    Direction::Down => Direction::Right,
    Direction::Left => Direction::Up,
    Direction::Right => Direction::Down,
  };
  next_beam(
    Beam {
      x: beam.x,
      y: beam.y,
      direction: new_direction,
    },
    grid,
  )
}

fn split_vertical(beam: Beam, grid: &Vec<Vec<char>>) -> Vec<Beam> {
  match beam.direction {
    Direction::Left | Direction::Right => vec![
      next_beam(
        Beam {
          x: beam.x,
          y: beam.y,
          direction: Direction::Up,
        },
        grid,
      ),
      next_beam(
        Beam {
          x: beam.x,
          y: beam.y,
          direction: Direction::Down,
        },
        grid,
      ),
    ],
    _ => vec![next_beam(beam, grid)],
  }
}

fn split_horizontal(beam: Beam, grid: &Vec<Vec<char>>) -> Vec<Beam> {
  match beam.direction {
    Direction::Up | Direction::Down => vec![
      next_beam(
        Beam {
          x: beam.x,
          y: beam.y,
          direction: Direction::Left,
        },
        grid,
      ),
      next_beam(
        Beam {
          x: beam.x,
          y: beam.y,
          direction: Direction::Right,
        },
        grid,
      ),
    ],
    _ => vec![next_beam(beam, grid)],
  }
}

fn main() {
  dbg!(solve("input/d16_full.txt", 1));
  dbg!(solve("input/d16_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve("input/d16_simple.txt", 1), 46);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d16_full.txt", 1), 7060);
  }

  #[test]
  fn test_part2_simple() {
    assert_eq!(solve("input/d16_simple.txt", 2), 51);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d16_full.txt", 2), 7493);
  }
}
