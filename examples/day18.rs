#[derive(Debug, Clone, Copy)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn from_char(c: char) -> Option<Self> {
    match c {
      'U' => Some(Direction::Up),
      'D' => Some(Direction::Down),
      'L' => Some(Direction::Left),
      'R' => Some(Direction::Right),
      _ => None,
    }
  }

  fn from_digit(d: u8) -> Option<Self> {
    match d {
      0 => Some(Direction::Right),
      1 => Some(Direction::Down),
      2 => Some(Direction::Left),
      3 => Some(Direction::Up),
      _ => None,
    }
  }
}

#[derive(Debug, Clone)]
struct Instruction {
  direction: Direction,
  distance: i64,
  #[allow(dead_code)]
  color: String,
}

fn parse_input(input: &str, part: usize) -> Vec<Instruction> {
  input
    .lines()
    .filter_map(|line| {
      let parts: Vec<&str> = line.split_whitespace().collect();
      if parts.len() == 3 {
        if part == 1 {
          Some(Instruction {
            direction: Direction::from_char(parts[0].chars().next()?).unwrap(),
            distance: parts[1].parse().ok()?,
            color: parts[2].trim_matches(|c| c == '(' || c == ')').to_string(),
          })
        } else {
          let color = parts[2].trim_matches(|c| c == '(' || c == '#' || c == ')');
          let distance = i64::from_str_radix(&color[..5], 16).ok()?;
          let direction = Direction::from_digit(color.chars().last()?.to_digit(16)? as u8)?;
          Some(Instruction {
            direction,
            distance,
            color: color.to_string(),
          })
        }
      } else {
        None
      }
    })
    .collect()
}

fn calculate_area(instructions: &[Instruction]) -> i64 {
  let mut x: i64 = 0;
  let mut y: i64 = 0;
  let mut area: i64 = 0;
  let mut perimeter: i64 = 0;

  for instruction in instructions {
    let next_x = match instruction.direction {
      Direction::Left => x - instruction.distance,
      Direction::Right => x + instruction.distance,
      _ => x,
    };
    let next_y = match instruction.direction {
      Direction::Up => y + instruction.distance,
      Direction::Down => y - instruction.distance,
      _ => y,
    };

    // Shoelace formula
    area += x * next_y - y * next_x;
    perimeter += instruction.distance;

    x = next_x;
    y = next_y;
  }

  // Apply Pick's theorem: A = i + b/2 - 1
  // Where A is the area, i is the number of interior points, and b is the number of boundary points
  // We want i + b, which is equal to A + b/2 + 1
  (area.abs() / 2) + (perimeter / 2) + 1
}

pub fn solve(input: &str, part: usize) -> i64 {
  let instructions = parse_input(input, part);
  calculate_area(&instructions)
}

fn main() {
  solve("input/d18_full.txt", 1);
  solve("input/d18_full.txt", 2);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    let input = fs::read_to_string("input/d18_simple.txt").expect("Failed to read input file");
    assert_eq!(solve(&input, 1), 62);
  }

  #[test]
  fn test_part1_full() {
    let input = fs::read_to_string("input/d18_full.txt").expect("Failed to read input file");
    assert_eq!(solve(&input, 1), 48652);
  }

  #[test]
  fn test_part2_simple() {
    let input = fs::read_to_string("input/d18_simple.txt").expect("Failed to read input file");
    assert_eq!(solve(&input, 2), 952408144115);
  }

  #[test]
  fn test_part2_full() {
    let input = fs::read_to_string("input/d18_full.txt").expect("Failed to read input file");
    assert_eq!(solve(&input, 2), 48652);
  }
}
