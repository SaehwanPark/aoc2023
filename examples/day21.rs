use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
  x: i64,
  y: i64,
}

impl Point {
  fn new(x: i64, y: i64) -> Self {
    Point { x, y }
  }

  fn neighbors(&self) -> Vec<Point> {
    vec![
      Point::new(self.x - 1, self.y),
      Point::new(self.x + 1, self.y),
      Point::new(self.x, self.y - 1),
      Point::new(self.x, self.y + 1),
    ]
  }
}

struct Garden {
  grid: Vec<Vec<char>>,
  start: Point,
  width: i64,
  height: i64,
}

impl Garden {
  fn from_input(input: &str) -> Self {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len() as i64;
    let width = grid[0].len() as i64;
    let start = grid
      .iter()
      .enumerate()
      .find_map(|(y, row)| {
        row
          .iter()
          .position(|&c| c == 'S')
          .map(|x| Point::new(x as i64, y as i64))
      })
      .expect("Start position not found");

    Garden {
      grid,
      start,
      width,
      height,
    }
  }

  fn is_garden_plot(&self, point: &Point) -> bool {
    let x = point.x.rem_euclid(self.width) as usize;
    let y = point.y.rem_euclid(self.height) as usize;
    self.grid[y][x] != '#'
  }

  fn count_reachable_plots(&self, steps: i64) -> i64 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut reachable = HashSet::new();

    queue.push_back((self.start, 0));
    visited.insert(self.start);

    while let Some((point, distance)) = queue.pop_front() {
      if distance % 2 == steps % 2 {
        reachable.insert(point);
      }

      if distance == steps {
        continue;
      }

      for neighbor in point.neighbors() {
        let wrapped_neighbor = Point::new(
          neighbor.x.rem_euclid(self.width),
          neighbor.y.rem_euclid(self.height),
        );
        if self.is_garden_plot(&wrapped_neighbor) && !visited.contains(&neighbor) {
          queue.push_back((neighbor, distance + 1));
          visited.insert(neighbor);
        }
      }
    }

    reachable.len() as i64
  }

  fn extrapolate_plots(&self, steps: i64) -> i64 {
    let half_width = self.width / 2;
    let step_increments: Vec<i64> = (0..=3).map(|i| half_width + self.width * i).collect();

    let reachable_counts: Vec<i64> = step_increments
      .iter()
      .map(|&steps| self.count_reachable_plots(steps))
      .collect();

    let mut diffs = vec![reachable_counts.clone()];
    while diffs.last().unwrap().iter().any(|&x| x != 0) {
      let new_diff: Vec<i64> = diffs
        .last()
        .unwrap()
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect();
      diffs.push(new_diff);
    }

    let num_step_cycles = (steps - half_width) / self.width;

    for _ in diffs.len()..=num_step_cycles as usize {
      for i in (0..diffs.len() - 1).rev() {
        let last = *diffs[i].last().unwrap();
        let next_last = *diffs[i + 1].last().unwrap();
        diffs[i].push(last + next_last);
      }
    }

    *diffs[0].last().unwrap()
  }
}

pub fn solve(input: &str, part: usize) -> i64 {
  let garden = Garden::from_input(input);
  match part {
    1 => garden.count_reachable_plots(64),
    2 => garden.extrapolate_plots(26501365),
    _ => panic!("Invalid part number"),
  }
}

fn main() {
  dbg!(solve("input/d21_full.txt", 1));
  dbg!(solve("input/d21_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  // #[test]
  // fn test_part1_simple() {
  //     let input = fs::read_to_string("input/d21_simple.txt").expect("Failed to read input file");
  //     let garden = Garden::from_input(&input);
  //     assert_eq!(garden.count_reachable_plots(6), 16);
  // }
  //
  // #[test]
  // fn test_part2_simple() {
  //     let input = fs::read_to_string("input/d21_simple.txt").expect("Failed to read input file");
  //     let garden = Garden::from_input(&input);
  //
  //     assert_eq!(garden.count_reachable_plots(6), 16);
  //     assert_eq!(garden.count_reachable_plots(10), 50);
  //     assert_eq!(garden.count_reachable_plots(50), 1594);
  //     assert_eq!(garden.count_reachable_plots(100), 6536);
  //     assert_eq!(garden.count_reachable_plots(500), 167004);
  //     assert_eq!(garden.count_reachable_plots(1000), 668697);
  //     assert_eq!(garden.count_reachable_plots(5000), 16733044);
  // }

  #[test]
  fn test_part2_full() {
    let input = fs::read_to_string("input/d21_full.txt").expect("Failed to read input file");
    assert_eq!(solve(&input, 2), 608152828731262);
  }
}
