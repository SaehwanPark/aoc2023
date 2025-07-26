use rand::prelude::*;
use std::collections::HashMap;
use std::fs;

type Graph = HashMap<String, Vec<String>>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn read_file(filename: &str) -> Result<Vec<String>> {
  let content = fs::read_to_string(filename)?;
  Ok(content.lines().map(|s| s.to_string()).collect())
}

struct Day25 {
  input: Vec<String>,
}

impl Day25 {
  fn new(input: Vec<String>) -> Self {
    Self { input }
  }

  fn solve_part_1(&self) -> i32 {
    let mut rng = rand::rng();

    loop {
      let mut graph = self.parse_input();
      let mut counts: HashMap<String, i32> = graph.keys().map(|k| (k.clone(), 1)).collect();

      while graph.len() > 2 {
        // Get random node 'a'
        let keys: Vec<_> = graph.keys().cloned().collect();
        let node_a = keys.choose(&mut rng).unwrap().clone();

        // Get random neighbor 'b' of 'a'
        let neighbors = &graph[&node_a];
        let node_b = neighbors.choose(&mut rng).unwrap().clone();

        // Create new super-node
        let new_node = format!("{}-{}", node_a, node_b);

        // Update counts
        let count_a = counts.remove(&node_a).unwrap_or(0);
        let count_b = counts.remove(&node_b).unwrap_or(0);
        counts.insert(new_node.clone(), count_a + count_b);

        // Combine and merge nodes
        self.combine_values(&mut graph, &node_a, &node_b, &new_node);
        self.merge_nodes(&mut graph, &node_a, &new_node);
        self.merge_nodes(&mut graph, &node_b, &new_node);
      }

      // Check if we found the minimum cut of size 3
      let remaining_keys: Vec<_> = graph.keys().collect();
      let (node_a, node_b) = (remaining_keys[0], remaining_keys[1]);

      if graph[node_a].len() == 3 {
        return counts[node_a] * counts[node_b];
      }
    }
  }

  fn combine_values(&self, graph: &mut Graph, node_a: &str, node_b: &str, new_node: &str) {
    let edges_a: Vec<_> = graph[node_a]
      .iter()
      .filter(|&neighbor| neighbor != node_b)
      .cloned()
      .collect();

    let edges_b: Vec<_> = graph[node_b]
      .iter()
      .filter(|&neighbor| neighbor != node_a)
      .cloned()
      .collect();

    let mut combined_edges = edges_a;
    combined_edges.extend(edges_b);

    graph.insert(new_node.to_string(), combined_edges);
  }

  fn merge_nodes(&self, graph: &mut Graph, old_node: &str, new_node: &str) {
    if let Some(neighbors) = graph.remove(old_node) {
      for target in neighbors {
        if let Some(target_neighbors) = graph.get_mut(&target) {
          for neighbor in target_neighbors.iter_mut() {
            if neighbor == old_node {
              *neighbor = new_node.to_string();
            }
          }
        }
      }
    }
  }

  fn parse_input(&self) -> Graph {
    let mut graph: Graph = HashMap::new();

    for line in &self.input {
      let parts: Vec<&str> = line.split(':').collect();
      let source_name = parts[0].trim();
      let connections: Vec<&str> = parts[1].trim().split_whitespace().collect();

      // Ensure source exists in graph
      graph
        .entry(source_name.to_string())
        .or_insert_with(Vec::new);

      for connection in connections {
        // Add bidirectional edges
        graph
          .entry(source_name.to_string())
          .or_insert_with(Vec::new)
          .push(connection.to_string());

        graph
          .entry(connection.to_string())
          .or_insert_with(Vec::new)
          .push(source_name.to_string());
      }
    }

    graph
  }
}

fn main() -> Result<()> {
  let input = read_file("input/d25_full.txt")?;
  let solver = Day25::new(input);
  let answer = solver.solve_part_1();
  println!("Part 1 answer: {}", answer);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = vec![
      "jqt: rhn xhk nvd".to_string(),
      "rsh: frs pzl lsr".to_string(),
      "xhk: hfx".to_string(),
      "cmg: qnr nvd lhk bvb".to_string(),
      "rhn: xhk bvb hfx".to_string(),
      "bvb: xhk hfx".to_string(),
      "pzl: lsr hfx nvd".to_string(),
      "qnr: nvd".to_string(),
      "ntq: jqt hfx bvb xhk".to_string(),
      "nvd: lhk".to_string(),
      "lsr: lhk".to_string(),
      "rzs: qnr cmg lsr rsh".to_string(),
      "frs: qnr lhk lsr".to_string(),
    ];

    let solver = Day25::new(input);
    let result = solver.solve_part_1();
    assert_eq!(result, 54);
  }
}
