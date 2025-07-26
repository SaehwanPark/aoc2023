use std::fs;

#[derive(Debug, Clone, Copy)]
struct Range {
  start: u64,
  end: u64,
}

impl Range {
  fn new(start: u64, length: u64) -> Self {
    Range {
      start,
      end: start + length,
    }
  }

  fn overlap(&self, other: &Range) -> Option<Range> {
    let start = self.start.max(other.start);
    let end = self.end.min(other.end);
    if start < end {
      Some(Range { start, end })
    } else {
      None
    }
  }
}

#[derive(Debug)]
struct Map {
  ranges: Vec<(Range, i64)>,
}

impl Map {
  fn apply(&self, input: Range) -> Vec<Range> {
    let mut result = vec![];
    let mut to_process = vec![input];

    while let Some(current) = to_process.pop() {
      let mut mapped = false;
      for &(range, offset) in &self.ranges {
        if let Some(overlap) = current.overlap(&range) {
          result.push(Range {
            start: (overlap.start as i64 + offset) as u64,
            end: (overlap.end as i64 + offset) as u64,
          });
          if overlap.start > current.start {
            to_process.push(Range {
              start: current.start,
              end: overlap.start,
            });
          }
          if current.end > overlap.end {
            to_process.push(Range {
              start: overlap.end,
              end: current.end,
            });
          }
          mapped = true;
          break;
        }
      }
      if !mapped {
        result.push(current);
      }
    }
    result
  }
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<Range>, Vec<Map>) {
  let mut sections = input.split("\n\n");
  let seed_numbers: Vec<u64> = sections
    .next()
    .unwrap()
    .split_whitespace()
    .skip(1)
    .map(|s| s.parse().unwrap())
    .collect();

  let seeds_part1: Vec<Range> = seed_numbers
    .iter()
    .map(|&seed| Range::new(seed, 1))
    .collect();

  let seeds_part2: Vec<Range> = seed_numbers
    .chunks(2)
    .map(|chunk| Range::new(chunk[0], chunk[1]))
    .collect();

  let maps = sections
    .map(|section| {
      let ranges = section
        .lines()
        .skip(1)
        .map(|line| {
          let mut nums = line.split_whitespace().map(|s| s.parse::<u64>().unwrap());
          let dest_start = nums.next().unwrap();
          let src_start = nums.next().unwrap();
          let length = nums.next().unwrap();
          (
            Range::new(src_start, length),
            dest_start as i64 - src_start as i64,
          )
        })
        .collect();
      Map { ranges }
    })
    .collect();

  (seeds_part1, seeds_part2, maps)
}

fn solve(input_file: &str, part: usize) -> u64 {
  let input = fs::read_to_string(input_file).unwrap();
  let (seeds_part1, seeds_part2, maps) = parse_input(&input);

  let seeds = if part == 1 { seeds_part1 } else { seeds_part2 };

  seeds
    .into_iter()
    .flat_map(|seed| {
      maps.iter().fold(vec![seed], |ranges, map| {
        ranges
          .into_iter()
          .flat_map(|range| map.apply(range))
          .collect()
      })
    })
    .map(|range| range.start)
    .min()
    .unwrap()
}

fn main() {
  dbg!(solve("input/d05_full.txt", 1));
  dbg!(solve("input/d05_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve("input/d05_simple.txt", 1), 35);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d05_full.txt", 1), 1181555926);
  }

  #[test]
  fn test_part2_simple() {
    assert_eq!(solve("input/d05_simple.txt", 2), 46);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d05_full.txt", 2), 37806486);
  }
}
