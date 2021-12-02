use extend::ext;
use itertools::Itertools;

#[ext]
impl str {
  fn prepare(&self) -> itertools::Tuples<std::str::SplitWhitespace<'_>, (&str, &str)> {
    self.split_whitespace().tuples::<(_, _)>()
  }
}

fn process((command, amount): (&str, &str), (x, y): (i32, i32)) -> (i32, i32) {
  let number_amount = amount.parse::<i32>().expect("not a number");

  match command {
    "forward" => {
      (x + number_amount, y)
    }
    "backward" => {
      (x - number_amount, y)
    }
    "up" => {
      (x, y - number_amount)
    }
    "down" => {
      (x, y + number_amount)
    }
    _ => {
      (x, y)
    }
  }
}

fn process_2((command, amount): (&str, &str), (x, y, aim): (i64, i64, i64)) -> (i64, i64, i64) {
  let number_amount = amount.parse::<i64>().expect("not a number");

  match command {
    "forward" => {
      (x + number_amount, y + (aim * number_amount), aim)
    }
    "up" => {
      (x, y, aim - number_amount)
    }
    "down" => {
      (x, y, aim + number_amount)
    }
    _ => {
      (x, y, aim)
    }
  }
}

fn day2_part1(input: &str) -> i32 {
  let (x, y) = input.prepare().fold((0, 0), |acc, elm| process(elm, acc));
  x * y
}

fn day2_part2(input: &str) -> i64 {
  let (x, y, _) = input.prepare().fold((0, 0, 0), |acc, elm| process_2(elm, acc));
  x * y
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn day2_part1_smoke() {
    let contents = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    let result = day2_part1(contents);
    assert_eq!(result, 150);
  }

  #[test]
  fn day2_part1_test() {
    let contents = fs::read_to_string("inputs/day2.txt").expect("couldn't read file");

    let result = day2_part1(&contents);
    assert_eq!(result, 1714950);
  }

  #[test]
  fn day2_part2_smoke() {
    let contents = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    let result = day2_part2(contents);
    assert_eq!(result, 900);
  }

  #[test]
  fn day2_part2_test() {
    let contents = fs::read_to_string("inputs/day2.txt").expect("couldn't read file");

    let result = day2_part2(&contents);
    assert_eq!(result, 1281977850);
  }
}
