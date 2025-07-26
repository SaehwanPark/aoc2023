use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug, Clone)]
struct Part {
  x: i32,
  m: i32,
  a: i32,
  s: i32,
}

#[derive(Debug, Clone)]
struct PartRange {
  x: Range<i32>,
  m: Range<i32>,
  a: Range<i32>,
  s: Range<i32>,
}

#[derive(Debug)]
enum Condition {
  GreaterThan(char, i32),
  LessThan(char, i32),
  Always,
}

#[derive(Debug)]
struct Rule {
  condition: Condition,
  destination: String,
}

#[derive(Debug)]
struct Workflow {
  #[allow(dead_code)]
  name: String,
  rules: Vec<Rule>,
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
  let mut workflows = HashMap::new();
  let mut parts = Vec::new();
  let mut parsing_workflows = true;

  for line in input.lines() {
    if line.is_empty() {
      parsing_workflows = false;
      continue;
    }

    if parsing_workflows {
      let (name, rules_str) = line.split_once('{').unwrap();
      let rules_str = rules_str.trim_end_matches('}');
      let rules = rules_str
        .split(',')
        .map(|rule_str| {
          if let Some((condition, destination)) = rule_str.split_once(':') {
            let condition = if condition.contains('>') {
              let (category, value) = condition.split_once('>').unwrap();
              Condition::GreaterThan(category.chars().next().unwrap(), value.parse().unwrap())
            } else {
              let (category, value) = condition.split_once('<').unwrap();
              Condition::LessThan(category.chars().next().unwrap(), value.parse().unwrap())
            };
            Rule {
              condition,
              destination: destination.to_string(),
            }
          } else {
            Rule {
              condition: Condition::Always,
              destination: rule_str.to_string(),
            }
          }
        })
        .collect();
      workflows.insert(
        name.to_string(),
        Workflow {
          name: name.to_string(),
          rules,
        },
      );
    } else {
      let part_str = line.trim_matches(|c| c == '{' || c == '}');
      let mut part = Part {
        x: 0,
        m: 0,
        a: 0,
        s: 0,
      };
      for rating in part_str.split(',') {
        let (category, value) = rating.split_once('=').unwrap();
        let value = value.parse().unwrap();
        match category {
          "x" => part.x = value,
          "m" => part.m = value,
          "a" => part.a = value,
          "s" => part.s = value,
          _ => panic!("Invalid category"),
        }
      }
      parts.push(part);
    }
  }

  (workflows, parts)
}

fn process_part(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
  let mut current_workflow = "in";
  loop {
    let workflow = workflows.get(current_workflow).unwrap();
    for rule in &workflow.rules {
      let condition_met = match rule.condition {
        Condition::GreaterThan(category, value) => match category {
          'x' => part.x > value,
          'm' => part.m > value,
          'a' => part.a > value,
          's' => part.s > value,
          _ => panic!("Invalid category"),
        },
        Condition::LessThan(category, value) => match category {
          'x' => part.x < value,
          'm' => part.m < value,
          'a' => part.a < value,
          's' => part.s < value,
          _ => panic!("Invalid category"),
        },
        Condition::Always => true,
      };

      if condition_met {
        match rule.destination.as_str() {
          "A" => return true,
          "R" => return false,
          _ => {
            current_workflow = &rule.destination;
            break;
          }
        }
      }
    }
  }
}

fn count_accepted_combinations(
  workflows: &HashMap<String, Workflow>,
  current: &str,
  mut ranges: PartRange,
) -> u64 {
  match current {
    "R" => 0,
    "A" => {
      ranges.x.len() as u64 * ranges.m.len() as u64 * ranges.a.len() as u64 * ranges.s.len() as u64
    }
    _ => {
      let workflow = workflows.get(current).unwrap();
      let mut total = 0;

      for rule in &workflow.rules {
        let (matching, non_matching) = split_range(&ranges, &rule.condition);
        total += count_accepted_combinations(workflows, &rule.destination, matching);
        ranges = non_matching;
      }

      total
    }
  }
}

fn split_range(range: &PartRange, condition: &Condition) -> (PartRange, PartRange) {
  let mut matching = range.clone();
  let mut non_matching = range.clone();

  match condition {
    Condition::GreaterThan(category, value) => {
      let (matching_range, non_matching_range) = match category {
        'x' => (&mut matching.x, &mut non_matching.x),
        'm' => (&mut matching.m, &mut non_matching.m),
        'a' => (&mut matching.a, &mut non_matching.a),
        's' => (&mut matching.s, &mut non_matching.s),
        _ => panic!("Invalid category"),
      };
      matching_range.start = (*value + 1).max(matching_range.start);
      non_matching_range.end = (*value + 1).min(non_matching_range.end);
    }
    Condition::LessThan(category, value) => {
      let (matching_range, non_matching_range) = match category {
        'x' => (&mut matching.x, &mut non_matching.x),
        'm' => (&mut matching.m, &mut non_matching.m),
        'a' => (&mut matching.a, &mut non_matching.a),
        's' => (&mut matching.s, &mut non_matching.s),
        _ => panic!("Invalid category"),
      };
      matching_range.end = *value.min(&matching_range.end);
      non_matching_range.start = *value.max(&non_matching_range.start);
    }
    Condition::Always => {
      return (
        range.clone(),
        PartRange {
          x: 0..0,
          m: 0..0,
          a: 0..0,
          s: 0..0,
        },
      );
    }
  }

  (matching, non_matching)
}

fn solve(input: &str, part: usize) -> u64 {
  let (workflows, parts) = parse_input(input);

  match part {
    1 => parts
      .iter()
      .filter(|part| process_part(part, &workflows))
      .map(|part| (part.x + part.m + part.a + part.s) as u64)
      .sum(),
    2 => {
      let initial_range = PartRange {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
      };
      count_accepted_combinations(&workflows, "in", initial_range)
    }
    _ => panic!("Invalid part number"),
  }
}

fn main() {
  dbg!(solve("input/d19_full.txt", 1));
  dbg!(solve("input/d19_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    let input = fs::read_to_string("input/d19_simple.txt").unwrap();
    assert_eq!(solve(&input, 1), 19114);
  }

  #[test]
  fn test_part1_full() {
    let input = fs::read_to_string("input/d19_full.txt").unwrap();
    assert_eq!(solve(&input, 1), 397061);
  }

  #[test]
  fn test_part2_simple() {
    let input = fs::read_to_string("input/d19_simple.txt").unwrap();
    assert_eq!(solve(&input, 2), 167409079868000);
  }

  #[test]
  fn test_part2_full() {
    let input = fs::read_to_string("input/d19_full.txt").unwrap();
    assert_eq!(solve(&input, 2), 125657431183201);
  }
}
