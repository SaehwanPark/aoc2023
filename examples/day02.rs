use std::collections::HashMap;
use std::fs::read_to_string;

pub fn solve(input_file: &str, part: usize) -> u32 {
  let content = read_to_string(input_file).expect("Failed to read input file");
  let games = parse_input(&content);

  match part {
    1 => {
      let max_cubes = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
      ]);

      games
        .iter()
        .filter(|game| is_game_possible(game, &max_cubes))
        .map(|game| game.id)
        .sum()
    }
    2 => games.iter().map(minimum_cubes_power).sum(),
    _ => panic!("Invalid part number"),
  }
}

#[derive(Debug)]
struct Game {
  id: u32,
  sets: Vec<HashMap<String, u32>>,
}

fn parse_input(input: &str) -> Vec<Game> {
  input
    .lines()
    .map(|line| {
      let mut parts = line.split(": ");
      let id = parts
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
      let sets = parts
        .next()
        .unwrap()
        .split("; ")
        .map(|set| {
          set
            .split(", ")
            .map(|cube| {
              let mut cube_parts = cube.split_whitespace();
              let count = cube_parts.next().unwrap().parse().unwrap();
              let color = cube_parts.next().unwrap().to_string();
              (color, count)
            })
            .collect()
        })
        .collect();
      Game { id, sets }
    })
    .collect()
}

fn is_game_possible(game: &Game, max_cubes: &HashMap<String, u32>) -> bool {
  game.sets.iter().all(|set| {
    set
      .iter()
      .all(|(color, &count)| count <= *max_cubes.get(color).unwrap_or(&0))
  })
}

fn minimum_cubes_power(game: &Game) -> u32 {
  let mut min_cubes = HashMap::new();
  for set in &game.sets {
    for (color, &count) in set {
      min_cubes
        .entry(color.clone())
        .and_modify(|e: &mut u32| *e = (*e).max(count))
        .or_insert(count);
    }
  }
  min_cubes.values().product()
}

fn main() {
  dbg!(solve("input/d02_full.txt", 1));
  dbg!(solve("input/d02_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve("input/d02_simple.txt", 1), 8);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d02_full.txt", 1), 2207);
  }

  #[test]
  fn test_part2_simple() {
    assert_eq!(solve("input/d02_simple.txt", 2), 2286);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d02_full.txt", 2), 62241);
  }
}
