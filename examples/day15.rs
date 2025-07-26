use std::fs;

fn hash(s: &str) -> usize {
  s.bytes().fold(0, |acc, b| ((acc + b as usize) * 17 % 256)) as usize
}

#[derive(Debug, Clone)]
struct Lens {
  label: String,
  focal_length: u32,
}

fn process_step(boxes: &mut Vec<Vec<Lens>>, step: &str) {
  let (label, operation) = step.split_at(step.find(['=', '-']).unwrap());
  let box_number = hash(label);

  match operation.chars().next().unwrap() {
    '-' => {
      if let Some(pos) = boxes[box_number]
        .iter()
        .position(|lens| lens.label == label)
      {
        boxes[box_number].remove(pos);
      }
    }
    '=' => {
      let focal_length = operation[1..].parse().unwrap();
      if let Some(pos) = boxes[box_number]
        .iter()
        .position(|lens| lens.label == label)
      {
        boxes[box_number][pos].focal_length = focal_length;
      } else {
        boxes[box_number].push(Lens {
          label: label.to_string(),
          focal_length,
        });
      }
    }
    _ => panic!("Invalid operation"),
  }
}

fn calculate_focusing_power(boxes: &Vec<Vec<Lens>>) -> u32 {
  boxes
    .iter()
    .enumerate()
    .flat_map(|(box_num, lenses)| {
      lenses
        .iter()
        .enumerate()
        .map(move |(slot, lens)| (box_num as u32 + 1) * (slot as u32 + 1) * lens.focal_length)
    })
    .sum()
}

pub fn solve(input_file: &str, part: usize) -> u32 {
  let content = fs::read_to_string(input_file).expect("Failed to read input file");
  let steps = content.trim().split(',');

  match part {
    1 => steps.map(|step| hash(step) as u32).sum(),
    2 => {
      let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
      steps.for_each(|step| process_step(&mut boxes, step));
      calculate_focusing_power(&boxes)
    }
    _ => panic!("Invalid part number"),
  }
}

fn main() {
  dbg!(solve("input/d15_full.txt", 1));
  dbg!(solve("input/d15_full.txt", 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hash() {
    assert_eq!(hash("HASH"), 52);
  }

  #[test]
  fn test_part1_simple() {
    assert_eq!(solve("input/d15_simple.txt", 1), 1320);
  }

  #[test]
  fn test_part1_full() {
    assert_eq!(solve("input/d15_full.txt", 1), 516070);
  }

  #[test]
  fn test_part2_simple() {
    assert_eq!(solve("input/d15_simple.txt", 2), 145);
  }

  #[test]
  fn test_part2_full() {
    assert_eq!(solve("input/d15_full.txt", 2), 244981);
  }
}
