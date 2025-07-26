use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

#[derive(Debug, Clone)]
struct Hand {
  cards: String,
  bid: u32,
  hand_type: HandType,
}

fn parse_input<P: AsRef<Path>>(path: P) -> io::Result<Vec<Hand>> {
  let file = File::open(path)?;
  let reader = io::BufReader::new(file);
  let mut hands = Vec::new();

  for line in reader.lines() {
    let line = line?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 2 {
      let cards = parts[0].to_string();
      let bid = parts[1].parse().unwrap();
      hands.push(Hand {
        cards,
        bid,
        hand_type: HandType::HighCard,
      }); // Placeholder type
    }
  }

  Ok(hands)
}

fn determine_hand_type_part1(cards: &str) -> HandType {
  let mut counts = HashMap::new();
  for card in cards.chars() {
    *counts.entry(card).or_insert(0) += 1;
  }

  let mut frequencies: Vec<_> = counts.values().cloned().collect();
  frequencies.sort_unstable_by(|a, b| b.cmp(a));

  match frequencies.as_slice() {
    [5] => HandType::FiveOfAKind,
    [4, 1] => HandType::FourOfAKind,
    [3, 2] => HandType::FullHouse,
    [3, 1, 1] => HandType::ThreeOfAKind,
    [2, 2, 1] => HandType::TwoPair,
    [2, 1, 1, 1] => HandType::OnePair,
    _ => HandType::HighCard,
  }
}

fn determine_hand_type_part2(cards: &str) -> HandType {
  let mut counts = HashMap::new();
  let mut joker_count = 0;

  for card in cards.chars() {
    if card == 'J' {
      joker_count += 1;
    } else {
      *counts.entry(card).or_insert(0) += 1;
    }
  }

  let mut frequencies: Vec<_> = counts.values().cloned().collect();
  frequencies.sort_unstable_by(|a, b| b.cmp(a));

  if !frequencies.is_empty() {
    frequencies[0] += joker_count;
  } else {
    frequencies.push(joker_count); // All jokers
  }

  match frequencies.as_slice() {
    [5] | [_] => HandType::FiveOfAKind,
    [4, 1] => HandType::FourOfAKind,
    [3, 2] => HandType::FullHouse,
    [3, 1, 1] => HandType::ThreeOfAKind,
    [2, 2, 1] => HandType::TwoPair,
    [2, 1, 1, 1] => HandType::OnePair,
    _ => HandType::HighCard,
  }
}

fn card_value_part1(card: char) -> u8 {
  match card {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' => 11,
    'T' => 10,
    _ => card.to_digit(10).unwrap() as u8,
  }
}

fn card_value_part2(card: char) -> u8 {
  match card {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'T' => 10,
    'J' => 1, // Joker is now the weakest card
    _ => card.to_digit(10).unwrap() as u8,
  }
}

fn compare_hands(a: &Hand, b: &Hand, card_value: fn(char) -> u8) -> Ordering {
  match a.hand_type.cmp(&b.hand_type) {
    Ordering::Equal => {
      for (card_a, card_b) in a.cards.chars().zip(b.cards.chars()) {
        match card_value(card_a).cmp(&card_value(card_b)) {
          Ordering::Equal => continue,
          other => return other,
        }
      }
      Ordering::Equal
    }
    other => other,
  }
}

fn calculate_winnings(hands: &mut [Hand]) -> u64 {
  hands
    .iter()
    .enumerate()
    .map(|(i, hand)| (i as u64 + 1) * hand.bid as u64)
    .sum()
}

fn solve_part1<P: AsRef<Path>>(input_path: P) -> io::Result<u64> {
  let mut hands = parse_input(input_path)?;
  for hand in &mut hands {
    hand.hand_type = determine_hand_type_part1(&hand.cards);
  }
  hands.sort_by(|a, b| compare_hands(a, b, card_value_part1));
  Ok(calculate_winnings(&mut hands))
}

fn solve_part2<P: AsRef<Path>>(input_path: P) -> io::Result<u64> {
  let mut hands = parse_input(input_path)?;
  for hand in &mut hands {
    hand.hand_type = determine_hand_type_part2(&hand.cards);
  }
  hands.sort_by(|a, b| compare_hands(a, b, card_value_part2));
  Ok(calculate_winnings(&mut hands))
}

fn main() {
  dbg!(solve_part1("input/d07_full.txt").unwrap());
  dbg!(solve_part2("input/d07_full.txt").unwrap());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_determine_hand_type_part1() {
    assert_eq!(determine_hand_type_part1("AAAAA"), HandType::FiveOfAKind);
    assert_eq!(determine_hand_type_part1("AA8AA"), HandType::FourOfAKind);
    assert_eq!(determine_hand_type_part1("23332"), HandType::FullHouse);
    assert_eq!(determine_hand_type_part1("TTT98"), HandType::ThreeOfAKind);
    assert_eq!(determine_hand_type_part1("23432"), HandType::TwoPair);
    assert_eq!(determine_hand_type_part1("A23A4"), HandType::OnePair);
    assert_eq!(determine_hand_type_part1("23456"), HandType::HighCard);
  }

  #[test]
  fn test_determine_hand_type_part2() {
    assert_eq!(determine_hand_type_part2("QJJQ2"), HandType::FourOfAKind);
    assert_eq!(determine_hand_type_part2("JJJJJ"), HandType::FiveOfAKind);
    assert_eq!(determine_hand_type_part2("JAAAA"), HandType::FiveOfAKind);
    assert_eq!(determine_hand_type_part2("JA234"), HandType::OnePair);
  }

  #[test]
  fn test_compare_hands_part1() {
    let hand1 = Hand {
      cards: "33332".to_string(),
      bid: 0,
      hand_type: HandType::FourOfAKind,
    };
    let hand2 = Hand {
      cards: "2AAAA".to_string(),
      bid: 0,
      hand_type: HandType::FourOfAKind,
    };
    assert_eq!(
      compare_hands(&hand1, &hand2, card_value_part1),
      Ordering::Greater
    );
  }

  #[test]
  fn test_compare_hands_part2() {
    let hand1 = Hand {
      cards: "JKKK2".to_string(),
      bid: 0,
      hand_type: HandType::FourOfAKind,
    };
    let hand2 = Hand {
      cards: "QQQQ2".to_string(),
      bid: 0,
      hand_type: HandType::FourOfAKind,
    };
    assert_eq!(
      compare_hands(&hand1, &hand2, card_value_part2),
      Ordering::Less
    );
  }

  #[test]
  fn test_total_winnings_part1_simple() {
    let result = solve_part1("input/d07_simple.txt").unwrap();
    assert_eq!(result, 6440);
  }

  #[test]
  fn test_total_winnings_part2_simple() {
    let result = solve_part2("input/d07_simple.txt").unwrap();
    assert_eq!(result, 5905);
  }

  #[test]
  fn test_total_winnings_part1() {
    let result = solve_part1("input/d07_full.txt").unwrap();
    assert_eq!(result, 251058093);
  }

  #[test]
  fn test_total_winnings_part2() {
    let result = solve_part2("input/d07_full.txt").unwrap();
    assert_eq!(result, 249781879);
  }
}
