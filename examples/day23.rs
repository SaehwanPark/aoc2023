use std::{
  collections::{HashMap, HashSet, VecDeque},
  fs,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos(usize, usize);

struct Map {
  tiles: Vec<Vec<char>>,
  width: usize,
  height: usize,
}

impl Map {
  fn new(input: &str) -> Self {
    let tiles: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = tiles.len();
    let width = tiles[0].len();
    Map {
      tiles,
      width,
      height,
    }
  }

  fn get_neighbors(&self, pos: Pos, ignore_slopes: bool) -> Vec<Pos> {
    let Pos(x, y) = pos;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    directions
      .iter()
      .filter_map(|&(dx, dy)| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
          let next_pos = Pos(nx as usize, ny as usize);
          let next_tile = self.tiles[ny as usize][nx as usize];
          if next_tile != '#' && (ignore_slopes || self.is_valid_move(pos, next_pos)) {
            Some(next_pos)
          } else {
            None
          }
        } else {
          None
        }
      })
      .collect()
  }

  fn is_valid_move(&self, from: Pos, to: Pos) -> bool {
    let Pos(fx, fy) = from;
    let Pos(tx, ty) = to;
    match self.tiles[fy][fx] {
      '>' => tx > fx,
      '<' => tx < fx,
      'v' => ty > fy,
      '^' => ty < fy,
      _ => true,
    }
  }
}

#[derive(Clone, Debug)]
struct Edge {
  to: Pos,
  distance: usize,
}

fn compress_graph(map: &Map, ignore_slopes: bool) -> HashMap<Pos, Vec<Edge>> {
  let mut graph = HashMap::new();
  let mut junctions = HashSet::new();
  junctions.insert(Pos(1, 0));
  junctions.insert(Pos(map.width - 2, map.height - 1));

  // Find all junctions
  for y in 0..map.height {
    for x in 0..map.width {
      if map.tiles[y][x] != '#' {
        let pos = Pos(x, y);
        if map.get_neighbors(pos, ignore_slopes).len() > 2 {
          junctions.insert(pos);
        }
      }
    }
  }

  // Connect junctions
  for &start in &junctions {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some((pos, distance)) = queue.pop_front() {
      if distance > 0 && junctions.contains(&pos) {
        graph
          .entry(start)
          .or_insert_with(Vec::new)
          .push(Edge { to: pos, distance });
        continue;
      }

      for neighbor in map.get_neighbors(pos, ignore_slopes) {
        if !visited.contains(&neighbor) {
          visited.insert(neighbor);
          queue.push_back((neighbor, distance + 1));
        }
      }
    }
  }

  graph
}

fn dfs(
  graph: &HashMap<Pos, Vec<Edge>>,
  pos: Pos,
  end: Pos,
  visited: &mut HashSet<Pos>,
  current_distance: usize,
) -> Option<usize> {
  if pos == end {
    return Some(current_distance);
  }

  visited.insert(pos);
  let mut max_distance = None;

  if let Some(edges) = graph.get(&pos) {
    for edge in edges {
      if !visited.contains(&edge.to) {
        if let Some(distance) = dfs(
          graph,
          edge.to,
          end,
          visited,
          current_distance + edge.distance,
        ) {
          max_distance = max_distance.max(Some(distance));
        }
      }
    }
  }

  visited.remove(&pos);
  max_distance
}

fn longest_hike(map: &Map, ignore_slopes: bool) -> usize {
  let graph = compress_graph(map, ignore_slopes);
  let start = Pos(1, 0);
  let end = Pos(map.width - 2, map.height - 1);
  let mut visited = HashSet::new();
  dfs(&graph, start, end, &mut visited, 0).unwrap_or(0)
}

fn solve(input: &str, part: usize) -> String {
  let map = Map::new(input);
  let result = longest_hike(&map, part == 2);
  result.to_string()
}

fn main() {
  let input = fs::read_to_string("input/d23_full.txt").expect("Failed to read input file");
  dbg!(solve(&input, 1));
  dbg!(solve(&input, 2));
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve(INPUT, 1), "94");
  }

  #[test]
  fn test_part2_simple() {
    assert_eq!(solve(INPUT, 2), "154");
  }

  #[test]
  fn test_part2_full() {
    let input = fs::read_to_string("input/d23_full.txt").expect("Failed to read input file");
    assert_eq!(solve(&input, 2), "6802"); // runs a bit long (~48s)
  }
}
