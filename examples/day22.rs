use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
  x1: i32,
  y1: i32,
  z1: i32,
  x2: i32,
  y2: i32,
  z2: i32,
}

impl Brick {
  fn from_str(s: &str) -> Self {
    let (start, end) = s.split_once('~').unwrap();
    let mut start_iter = start.split(',').map(|n| n.parse().unwrap());
    let mut end_iter = end.split(',').map(|n| n.parse().unwrap());
    Self {
      x1: start_iter.next().unwrap(),
      y1: start_iter.next().unwrap(),
      z1: start_iter.next().unwrap(),
      x2: end_iter.next().unwrap(),
      y2: end_iter.next().unwrap(),
      z2: end_iter.next().unwrap(),
    }
  }

  fn lowest_z(&self) -> i32 {
    min(self.z1, self.z2)
  }

  fn highest_z(&self) -> i32 {
    max(self.z1, self.z2)
  }

  fn fall_to(&mut self, new_z: i32) {
    let diff = self.lowest_z() - new_z;
    self.z1 -= diff;
    self.z2 -= diff;
  }
}

fn simulate_falling(bricks: &mut Vec<Brick>) {
  let mut height_map: HashMap<(i32, i32), i32> = HashMap::new();

  bricks.sort_by_key(|b| b.lowest_z());

  for brick in bricks.iter_mut() {
    let mut max_height = 0;
    for x in min(brick.x1, brick.x2)..=max(brick.x1, brick.x2) {
      for y in min(brick.y1, brick.y2)..=max(brick.y1, brick.y2) {
        max_height = max(max_height, *height_map.get(&(x, y)).unwrap_or(&0));
      }
    }

    brick.fall_to(max_height + 1);

    for x in min(brick.x1, brick.x2)..=max(brick.x1, brick.x2) {
      for y in min(brick.y1, brick.y2)..=max(brick.y1, brick.y2) {
        height_map.insert((x, y), brick.highest_z());
      }
    }
  }
}

fn build_support_graph(
  bricks: &[Brick],
) -> (
  HashMap<usize, HashSet<usize>>,
  HashMap<usize, HashSet<usize>>,
) {
  let mut supports = HashMap::new();
  let mut supported_by = HashMap::new();

  for (i, upper) in bricks.iter().enumerate() {
    for (j, lower) in bricks.iter().enumerate() {
      if i != j && upper.lowest_z() == lower.highest_z() + 1 {
        if (min(upper.x1, upper.x2)..=max(upper.x1, upper.x2)).any(|x| {
          (min(lower.x1, lower.x2)..=max(lower.x1, lower.x2)).contains(&x)
            && (min(upper.y1, upper.y2)..=max(upper.y1, upper.y2))
              .any(|y| (min(lower.y1, lower.y2)..=max(lower.y1, lower.y2)).contains(&y))
        }) {
          supports.entry(j).or_insert(HashSet::new()).insert(i);
          supported_by.entry(i).or_insert(HashSet::new()).insert(j);
        }
      }
    }
  }

  (supports, supported_by)
}

fn count_falling_bricks(
  i: usize,
  supports: &HashMap<usize, HashSet<usize>>,
  supported_by: &HashMap<usize, HashSet<usize>>,
) -> usize {
  let mut falling = HashSet::new();
  let mut queue = VecDeque::new();
  queue.push_back(i);
  falling.insert(i);

  while let Some(brick) = queue.pop_front() {
    if let Some(supported_bricks) = supports.get(&brick) {
      for &supported in supported_bricks {
        if supported_by[&supported].is_subset(&falling) {
          falling.insert(supported);
          queue.push_back(supported);
        }
      }
    }
  }

  falling.len() - 1 // Subtract 1 to exclude the initially disintegrated brick
}

fn solve(input: &str, part: usize) -> String {
  let mut bricks: Vec<Brick> = input.lines().map(Brick::from_str).collect();
  simulate_falling(&mut bricks);
  let (supports, supported_by) = build_support_graph(&bricks);

  match part {
    1 => {
      let safe_bricks: Vec<usize> = (0..bricks.len())
        .filter(|&i| {
          let is_safe = supports.get(&i).map_or(true, |supported_bricks| {
            supported_bricks.iter().all(|&j| {
              supported_by
                .get(&j)
                .map_or(false, |supporters| supporters.len() > 1)
            })
          });
          println!("Brick {} is safe to disintegrate: {}", i, is_safe);
          is_safe
        })
        .collect();

      println!("Part 1 Debug - Safe to disintegrate: {:?}", safe_bricks);
      safe_bricks.len().to_string()
    }
    2 => {
      let total_falling: usize = (0..bricks.len())
        .map(|i| count_falling_bricks(i, &supports, &supported_by))
        .sum();
      println!("Part 2 Debug - Total falling bricks: {}", total_falling);
      total_falling.to_string()
    }
    _ => "Invalid part".to_string(),
  }
}

fn main() {
  dbg!(solve("input/d22_full.txt", 1));
  dbg!(solve("input/d22_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  const TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

  #[test]
  fn test_part1_simple() {
    println!("Running Part 1 test...");
    let mut bricks: Vec<Brick> = TEST_INPUT.lines().map(Brick::from_str).collect();
    simulate_falling(&mut bricks);
    let (supports, supported_by) = build_support_graph(&bricks);
    println!("Supports: {:?}", supports);
    println!("Supported by: {:?}", supported_by);
    let result = solve(TEST_INPUT, 1);
    println!("Part 1 test result: {}", result);
    assert_eq!(result, "5");
  }

  #[test]
  fn test_part2_simple() {
    println!("Running Part 2 test...");
    let result = solve(TEST_INPUT, 2);
    println!("Part 2 test result: {}", result);
    assert_eq!(result, "7");
  }

  #[test]
  fn test_part2_full() {
    let input = fs::read_to_string("input/d22_full.txt").expect("Failed to read input file");
    assert_eq!(solve(&input, 2), "74287");
  }
}
