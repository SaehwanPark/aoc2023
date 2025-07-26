use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
  x: usize,
  y: usize,
}

fn solve(input_file: &str, part: usize) -> usize {
  let content = fs::read_to_string(input_file).expect("Failed to read input file");
  let universe: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

  let empty_rows: Vec<usize> = universe
    .iter()
    .enumerate()
    .filter(|(_, row)| row.iter().all(|&c| c == '.'))
    .map(|(i, _)| i)
    .collect();

  let empty_cols: Vec<usize> = (0..universe[0].len())
    .filter(|&col| universe.iter().all(|row| row[col] == '.'))
    .collect();

  let expansion_factor = if part == 1 { 2 } else { 1000000 };

  let galaxies: Vec<Point> = get_galaxies(&universe, &empty_rows, &empty_cols, expansion_factor);

  let mut total_distance = 0;

  for i in 0..galaxies.len() {
    for j in (i + 1)..galaxies.len() {
      let distance = manhattan_distance(galaxies[i], galaxies[j]);
      total_distance += distance;
    }
  }

  total_distance
}

fn get_galaxies(
  universe: &Vec<Vec<char>>,
  empty_rows: &[usize],
  empty_cols: &[usize],
  expansion_factor: usize,
) -> Vec<Point> {
  universe
    .iter()
    .enumerate()
    .flat_map(|(y, row)| {
      row.iter().enumerate().filter_map(move |(x, &c)| {
        if c == '#' {
          let expanded_x =
            x + empty_cols.iter().take_while(|&&col| col < x).count() * (expansion_factor - 1);
          let expanded_y =
            y + empty_rows.iter().take_while(|&&row| row < y).count() * (expansion_factor - 1);
          Some(Point {
            x: expanded_x,
            y: expanded_y,
          })
        } else {
          None
        }
      })
    })
    .collect()
}

fn manhattan_distance(a: Point, b: Point) -> usize {
  a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn main() {
  dbg!(solve("input/d11_full.txt", 1));
  dbg!(solve("input/d11_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve("input/d11_simple.txt", 1), 374);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d11_full.txt", 1), 9545480);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d11_full.txt", 2), 406725732046);
  }
}
