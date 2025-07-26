use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone)]
enum ModuleType {
  FlipFlop(bool),
  Conjunction(HashMap<String, bool>),
  Broadcaster,
}

#[derive(Debug, Clone)]
struct Module {
  module_type: ModuleType,
  destinations: Vec<String>,
}

fn parse_input(input: &str) -> HashMap<String, Module> {
  let mut modules = HashMap::new();

  for line in input.lines() {
    let parts: Vec<&str> = line.split(" -> ").collect();
    let (name, module_type) = match parts[0].chars().next().unwrap() {
      '%' => (&parts[0][1..], ModuleType::FlipFlop(false)),
      '&' => (&parts[0][1..], ModuleType::Conjunction(HashMap::new())),
      'b' => ("broadcaster", ModuleType::Broadcaster),
      _ => panic!("Unknown module type"),
    };

    let destinations: Vec<String> = parts[1].split(", ").map(String::from).collect();
    modules.insert(
      name.to_string(),
      Module {
        module_type,
        destinations,
      },
    );
  }

  // Initialize conjunction modules
  let module_names: Vec<String> = modules.keys().cloned().collect();
  for name in module_names {
    let destinations = modules[&name].destinations.clone();
    for dest in destinations {
      if let Some(Module {
        module_type: ModuleType::Conjunction(memory),
        ..
      }) = modules.get_mut(&dest)
      {
        memory.insert(name.clone(), false);
      }
    }
  }

  modules
}

fn push_button(
  modules: &mut HashMap<String, Module>,
  watch_list: &HashSet<String>,
) -> (usize, usize, HashMap<String, bool>) {
  let mut queue = VecDeque::new();
  queue.push_back(("button".to_string(), "broadcaster".to_string(), false));

  let mut low_count = 0;
  let mut high_count = 0;
  let mut pulses = HashMap::new();

  while let Some((from, to, pulse)) = queue.pop_front() {
    if pulse {
      high_count += 1;
    } else {
      low_count += 1;
    }

    if watch_list.contains(&from) && pulse {
      pulses.insert(from.clone(), true);
    }

    if let Some(module) = modules.get_mut(&to) {
      match &mut module.module_type {
        ModuleType::FlipFlop(state) => {
          if !pulse {
            *state = !*state;
            for dest in &module.destinations {
              queue.push_back((to.clone(), dest.clone(), *state));
            }
          }
        }
        ModuleType::Conjunction(memory) => {
          memory.insert(from, pulse);
          let output = !memory.values().all(|&v| v);
          for dest in &module.destinations {
            queue.push_back((to.clone(), dest.clone(), output));
          }
        }
        ModuleType::Broadcaster => {
          for dest in &module.destinations {
            queue.push_back((to.clone(), dest.clone(), pulse));
          }
        }
      }
    }
  }

  (low_count, high_count, pulses)
}

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
  a * b / gcd(a, b)
}

fn find_cycles(
  modules: &mut HashMap<String, Module>,
  watch_list: &HashSet<String>,
) -> HashMap<String, usize> {
  let mut cycles = HashMap::new();
  let mut press_count = 0;

  while cycles.len() < watch_list.len() {
    press_count += 1;
    let (_, _, pulses) = push_button(modules, watch_list);

    for (module, high_pulse) in pulses {
      if high_pulse && !cycles.contains_key(&module) {
        cycles.insert(module, press_count);
      }
    }
  }

  cycles
}

fn solve_part_two(modules: &mut HashMap<String, Module>) -> usize {
  let rx_input = modules
    .iter()
    .find(|(_, module)| module.destinations.contains(&"rx".to_string()))
    .map(|(name, _)| name.clone())
    .expect("No module leads to 'rx'");

  let watch_list: HashSet<String> = modules
    .iter()
    .filter(|(_, module)| module.destinations.contains(&rx_input))
    .map(|(name, _)| name.clone())
    .collect();

  let cycles = find_cycles(modules, &watch_list);

  cycles.values().fold(1, |acc, &cycle| lcm(acc, cycle))
}

pub fn solve(input_file: &str, part: usize) -> usize {
  let input = fs::read_to_string(input_file).expect("Failed to read input file");
  let mut modules = parse_input(&input);

  match part {
    1 => {
      let mut total_low = 0;
      let mut total_high = 0;
      for _ in 0..1000 {
        let (low, high, _) = push_button(&mut modules, &HashSet::new());
        total_low += low;
        total_high += high;
      }
      total_low * total_high
    }
    2 => solve_part_two(&mut modules),
    _ => panic!("Invalid part number"),
  }
}

fn main() {
  dbg!(solve("input/d20_full.txt", 1));
  dbg!(solve("input/d20_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d20_full.txt", 1), 712543680);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d20_full.txt", 2), 238920142622879);
  }
}
