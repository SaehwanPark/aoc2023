use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
  heat_loss: i32,
  row: usize,
  col: usize,
  direction: (i32, i32),
  steps: usize,
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other.heat_loss.cmp(&self.heat_loss)
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
  input
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
    })
    .collect()
}

fn solve(input: &str, part: usize) -> i32 {
  let grid = parse_input(input);
  let rows = grid.len();
  let cols = grid[0].len();

  let mut heap = BinaryHeap::new();
  let mut visited = HashMap::new();

  let start_state = State {
    heat_loss: 0,
    row: 0,
    col: 0,
    direction: (0, 0),
    steps: 0,
  };

  heap.push(start_state);

  while let Some(state) = heap.pop() {
    if state.row == rows - 1 && state.col == cols - 1 {
      return state.heat_loss;
    }

    let key = (state.row, state.col, state.direction, state.steps);
    if visited.contains_key(&key) && visited[&key] <= state.heat_loss {
      continue;
    }
    visited.insert(key, state.heat_loss);

    let directions = if state.direction == (0, 0) {
      vec![(0, 1), (1, 0)]
    } else {
      vec![
        (state.direction.1, -state.direction.0),
        (-state.direction.1, state.direction.0),
        state.direction,
      ]
    };

    for &dir in &directions {
      let new_row = state.row as i32 + dir.0;
      let new_col = state.col as i32 + dir.1;

      if new_row < 0 || new_row >= rows as i32 || new_col < 0 || new_col >= cols as i32 {
        continue;
      }

      let new_row = new_row as usize;
      let new_col = new_col as usize;

      let new_steps = if dir == state.direction {
        state.steps + 1
      } else {
        1
      };

      let min_steps = if part == 1 { 0 } else { 4 };
      let max_steps = if part == 1 { 3 } else { 10 };

      if new_steps > max_steps
        || (dir != state.direction && state.steps < min_steps && state.direction != (0, 0))
      {
        continue;
      }

      let new_state = State {
        heat_loss: state.heat_loss + grid[new_row][new_col],
        row: new_row,
        col: new_col,
        direction: dir,
        steps: new_steps,
      };

      heap.push(new_state);
    }
  }

  -1 // No path found
}

fn main() {
  solve("input/d17_full.txt", 1);
  solve("input/d17_full.txt", 2);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    let input = fs::read_to_string("input/d17_simple.txt").unwrap();
    assert_eq!(solve(&input, 1), 102);
  }

  #[test]
  fn test_part1_full() {
    let input = fs::read_to_string("input/d17_full.txt").unwrap();
    assert_eq!(solve(&input, 1), 1246);
  }

  #[test]
  fn test_part2_simple() {
    let input = fs::read_to_string("input/d17_simple.txt").unwrap();
    assert_eq!(solve(&input, 2), 94);
  }

  #[test]
  fn test_part2_full() {
    let input = fs::read_to_string("input/d17_full.txt").unwrap();
    assert_eq!(solve(&input, 2), 1389);
  }
}
