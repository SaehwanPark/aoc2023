use rand::prelude::*; // Add this line
use std::ops::RangeInclusive; // Add this line

#[derive(Debug, Clone, Copy)]
struct Point3D {
  x: i64,
  y: i64,
  z: i64,
}

impl Point3D {
  fn from_str(input: &str) -> Result<Self, String> {
    let coords: Vec<i64> = input
      .split(',')
      .map(|s| {
        s.trim()
          .parse()
          .map_err(|e| format!("Invalid coordinate: {}", e))
      })
      .collect::<Result<Vec<i64>, String>>()?;

    if coords.len() != 3 {
      return Err(format!("Expected 3 coordinates, got {}", coords.len()));
    }

    Ok(Point3D {
      x: coords[0],
      y: coords[1],
      z: coords[2],
    })
  }
}

#[derive(Debug, Clone)]
struct Hailstone {
  position: Point3D,
  velocity: Point3D,
  slope: Option<f64>,
}

impl Hailstone {
  fn from_str(input: &str) -> Result<Self, String> {
    let parts: Vec<&str> = input.split('@').collect();
    if parts.len() != 2 {
      return Err("Invalid hailstone format".to_string());
    }

    let position = Point3D::from_str(parts[0])?;
    let velocity = Point3D::from_str(parts[1])?;

    let slope = if velocity.x == 0 {
      None
    } else {
      Some(velocity.y as f64 / velocity.x as f64)
    };

    Ok(Hailstone {
      position,
      velocity,
      slope,
    })
  }

  fn with_velocity_delta(&self, vx: i64, vy: i64) -> Self {
    let new_velocity = Point3D {
      x: self.velocity.x + vx,
      y: self.velocity.y + vy,
      z: self.velocity.z,
    };

    let new_slope = if new_velocity.x == 0 {
      None
    } else {
      Some(new_velocity.y as f64 / new_velocity.x as f64)
    };

    Hailstone {
      position: self.position,
      velocity: new_velocity,
      slope: new_slope,
    }
  }

  fn predict_z(&self, time: f64, delta_vz: i64) -> f64 {
    self.position.z as f64 + time * (self.velocity.z + delta_vz) as f64
  }

  fn intersection_with(&self, other: &Hailstone) -> Option<Intersection> {
    match (self.slope, other.slope) {
      (Some(slope), Some(other_slope)) if slope != other_slope => {
        let c = self.position.y as f64 - slope * self.position.x as f64;
        let other_c = other.position.y as f64 - other_slope * other.position.x as f64;

        let x = (other_c - c) / (slope - other_slope);
        let t1 = (x - self.position.x as f64) / self.velocity.x as f64;
        let t2 = (x - other.position.x as f64) / other.velocity.x as f64;

        if t1 < 0.0 || t2 < 0.0 {
          None
        } else {
          let y = slope * (x - self.position.x as f64) + self.position.y as f64;
          Some(Intersection { x, y, time: t1 })
        }
      }
      _ => None,
    }
  }
}

#[derive(Debug, Clone, Copy)]
struct Intersection {
  x: f64,
  y: f64,
  time: f64,
}

fn parse_input(input: &str) -> Result<Vec<Hailstone>, String> {
  input.lines().map(Hailstone::from_str).collect()
}

fn solve_part1(hailstones: &[Hailstone], range: &RangeInclusive<f64>) -> usize {
  hailstones
    .iter()
    .enumerate()
    .flat_map(|(i, h1)| {
      hailstones[i + 1..].iter().filter_map(move |h2| {
        h1.intersection_with(h2)
          .filter(|intersection| range.contains(&intersection.x) && range.contains(&intersection.y))
      })
    })
    .count()
}

fn solve_part2(hailstones: &[Hailstone]) -> i64 {
  let range = -500..=500;
  let mut rng = rand::rng();

  loop {
    let hail: Vec<_> = hailstones.choose_multiple(&mut rng, 4).cloned().collect();
    for delta_x in range.clone() {
      for delta_y in range.clone() {
        let hail0 = hail[0].with_velocity_delta(delta_x, delta_y);
        let intercepts: Vec<_> = hail
          .iter()
          .skip(1)
          .filter_map(|h| {
            h.with_velocity_delta(delta_x, delta_y)
              .intersection_with(&hail0)
          })
          .collect();

        if intercepts.len() == 3
          && intercepts
            .iter()
            .all(|i| (i.x - intercepts[0].x).abs() < f64::EPSILON)
          && intercepts
            .iter()
            .all(|i| (i.y - intercepts[0].y).abs() < f64::EPSILON)
        {
          for delta_z in range.clone() {
            let z1 = hail[1].predict_z(intercepts[0].time, delta_z);
            let z2 = hail[2].predict_z(intercepts[1].time, delta_z);
            let z3 = hail[3].predict_z(intercepts[2].time, delta_z);

            if (z1 - z2).abs() < f64::EPSILON && (z2 - z3).abs() < f64::EPSILON {
              return (intercepts[0].x + intercepts[0].y + z1).round() as i64;
            }
          }
        }
      }
    }
  }
}

pub fn solve(input: &str, part: usize) -> Result<String, String> {
  let hailstones = parse_input(input)?;
  match part {
    1 => {
      let range = 200000000000000.0..=400000000000000.0;
      Ok(solve_part1(&hailstones, &range).to_string())
    }
    2 => Ok(solve_part2(&hailstones).to_string()),
    _ => Err(format!("Invalid part number: {}", part)),
  }
}

fn main() -> Result<(), String> {
  dbg!(solve("input/d24_full.txt", 1)?);
  dbg!(solve("input/d24_full.txt", 2)?);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn test_part1_simple() {
    let input = "19, 13, 30 @ -2,  1, -2\n18, 19, 22 @ -1, -1, -2\n20, 25, 34 @ -2, -2, -4\n12, 31, 28 @ -1, -2, -1\n20, 19, 15 @  1, -5, -3";
    let range = 7.0..=27.0;
    let hailstones = parse_input(input).unwrap();
    assert_eq!(solve_part1(&hailstones, &range).to_string(), "2");
  }

  #[test]
  fn test_part2_simple() {
    let input = "19, 13, 30 @ -2,  1, -2\n18, 19, 22 @ -1, -1, -2\n20, 25, 34 @ -2, -2, -4\n12, 31, 28 @ -1, -2, -1\n20, 19, 15 @  1, -5, -3";
    assert_eq!(solve(input, 2).unwrap(), "47");
  }

  #[test]
  fn test_part2_full() {
    let input = fs::read_to_string("input/d24_full.txt").expect("Failed to read file.");
    assert_eq!(solve(&input, 2).unwrap(), "1025127405449117");
  }
}
