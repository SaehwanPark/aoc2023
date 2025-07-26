use std::collections::HashMap;
use std::fs;
use std::io;

fn parse_input(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
  let mut lines = input.lines();
  let instructions: Vec<char> = lines.next().unwrap().chars().collect();

  let mut network = HashMap::new();
  for line in lines.skip(1) {
    let parts: Vec<&str> = line
      .split(&['=', '(', ')', ','])
      .map(|s| s.trim())
      .filter(|s| !s.is_empty())
      .collect();
    if parts.len() == 3 {
      network.insert(
        parts[0].to_string(),
        (parts[1].to_string(), parts[2].to_string()),
      );
    }
  }

  (instructions, network)
}

fn navigate_network(instructions: &[char], network: &HashMap<String, (String, String)>) -> usize {
  let mut current_node = "AAA".to_string();
  let mut steps = 0;
  let mut instruction_index = 0;

  while current_node != "ZZZ" {
    let (left, right) = network.get(&current_node).unwrap();
    current_node = match instructions[instruction_index] {
      'L' => left.clone(),
      'R' => right.clone(),
      _ => panic!("Invalid instruction"),
    };

    steps += 1;
    instruction_index = (instruction_index + 1) % instructions.len();
  }

  steps
}

fn navigate_network_ghost(
  instructions: &[char],
  network: &HashMap<String, (String, String)>,
) -> usize {
  let start_nodes: Vec<String> = network
    .keys()
    .filter(|k| k.ends_with('A'))
    .cloned()
    .collect();

  let mut cycles: Vec<usize> = Vec::new();

  for start_node in start_nodes {
    let mut current_node = start_node;
    let mut steps = 0;
    let mut instruction_index = 0;

    while !current_node.ends_with('Z') {
      let (left, right) = network.get(&current_node).unwrap();
      current_node = match instructions[instruction_index] {
        'L' => left.clone(),
        'R' => right.clone(),
        _ => panic!("Invalid instruction"),
      };

      steps += 1;
      instruction_index = (instruction_index + 1) % instructions.len();
    }

    cycles.push(steps);
  }

  lcm(&cycles)
}

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(numbers: &[usize]) -> usize {
  numbers
    .iter()
    .fold(1, |acc, &num| acc * num / gcd(acc, num))
}

fn solve_from_file(filename: &str, part: usize) -> io::Result<usize> {
  let input = fs::read_to_string(filename)?;
  let (instructions, network) = parse_input(&input);
  Ok(match part {
    1 => navigate_network(&instructions, &network),
    2 => navigate_network_ghost(&instructions, &network),
    _ => panic!("Invalid part number"),
  })
}

fn main() -> io::Result<()> {
  dbg!(solve_from_file("input/d08_full.txt", 1)?);
  dbg!(solve_from_file("input/d08_full.txt", 2)?);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() -> io::Result<()> {
    assert_eq!(solve_from_file("input/d08_simple1.txt", 1)?, 2);
    Ok(())
  }

  #[test]
  fn test_example_2() -> io::Result<()> {
    assert_eq!(solve_from_file("input/d08_simple2.txt", 1)?, 6);
    Ok(())
  }

  #[test]
  fn test_example_3() -> io::Result<()> {
    assert_eq!(solve_from_file("input/d08_simple3.txt", 2)?, 6);
    Ok(())
  }

  #[test]
  fn test_full_pt1() -> io::Result<()> {
    assert_eq!(solve_from_file("input/d08_full.txt", 1)?, 14681);
    Ok(())
  }

  #[test]
  fn test_full_pt2() -> io::Result<()> {
    assert_eq!(solve_from_file("input/d08_full.txt", 2)?, 14321394058031);
    Ok(())
  }
}
