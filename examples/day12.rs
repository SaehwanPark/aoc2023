use std::collections::HashMap;
use std::fs;

fn solve(input_file: &str, part: usize) -> usize {
  let content = fs::read_to_string(input_file).expect("Failed to read file");
  let lines: Vec<&str> = content.lines().collect();

  lines
    .iter()
    .map(|line| count_arrangements(line, part))
    .sum()
}

fn count_arrangements(line: &str, part: usize) -> usize {
  let (springs, groups) = parse_line(line, part);
  let mut memo = HashMap::new();
  count_arrangements_recursive(&springs, &groups, 0, 0, 0, &mut memo)
}

fn parse_line(line: &str, part: usize) -> (String, Vec<usize>) {
  let parts: Vec<&str> = line.split_whitespace().collect();
  let mut springs = parts[0].to_string();
  let mut groups: Vec<usize> = parts[1].split(',').map(|n| n.parse().unwrap()).collect();

  if part == 2 {
    springs = vec![springs.clone(); 5].join("?");
    groups = groups.repeat(5);
  }

  (springs, groups)
}

fn count_arrangements_recursive(
  springs: &str,
  groups: &[usize],
  spring_index: usize,
  group_index: usize,
  current_group_size: usize,
  memo: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
  let key = (spring_index, group_index, current_group_size);
  if let Some(&count) = memo.get(&key) {
    return count;
  }

  if spring_index == springs.len() {
    return if group_index == groups.len() && current_group_size == 0 {
      1
    } else if group_index == groups.len() - 1 && groups[group_index] == current_group_size {
      1
    } else {
      0
    };
  }

  let mut count = 0;
  let spring = springs.chars().nth(spring_index).unwrap();

  if spring == '.' || spring == '?' {
    if current_group_size == 0 {
      count +=
        count_arrangements_recursive(springs, groups, spring_index + 1, group_index, 0, memo);
    } else if group_index < groups.len() && groups[group_index] == current_group_size {
      count +=
        count_arrangements_recursive(springs, groups, spring_index + 1, group_index + 1, 0, memo);
    }
  }

  if spring == '#' || spring == '?' {
    count += count_arrangements_recursive(
      springs,
      groups,
      spring_index + 1,
      group_index,
      current_group_size + 1,
      memo,
    );
  }

  memo.insert(key, count);
  count
}

fn main() {
  dbg!(solve("input/d12_full.txt", 1));
  dbg!(solve("input/d12_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve("input/d12_simple.txt", 1), 21);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d12_full.txt", 1), 7622);
  }

  #[test]
  fn test_part2_simple() {
    assert_eq!(solve("input/d12_simple.txt", 2), 525152);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d12_full.txt", 2), 4964259839627);
  }
}
