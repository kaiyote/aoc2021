use extend::ext;
use itertools::Itertools;

#[ext]
impl str {
    fn prepare(&self) -> std::iter::Map<std::str::SplitWhitespace<'_>, for<'r> fn(&'r str) -> i32> {
        self.split_whitespace().map(|s| s.parse::<i32>().unwrap())
    }
}

#[ext]
impl<T: Sized + Iterator<Item = i32>> T {
    fn get_answer(&mut self) -> i32 {
        self.fold((i32::MAX, 0), |(prev, count), elem| {
            (elem, if elem > prev { count + 1 } else { count })
        })
        .1
    }
}

fn day1_part1(input: &str) -> i32 {
    input.prepare().get_answer()
}

fn day1_part2(input: &str) -> i32 {
    input
        .prepare()
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .get_answer()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn day1_part1_smoke() {
        let contents = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

        let result = day1_part1(contents);
        assert_eq!(result, 7);
    }

    #[test]
    fn day1_part1_test() {
        let contents = fs::read_to_string("inputs/day1.txt").expect("couldn't read file");

        let result = day1_part1(&contents);
        assert_eq!(result, 1393);
    }

    #[test]
    fn day1_part2_smoke() {
        let contents = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

        let result = day1_part2(contents);
        assert_eq!(result, 5);
    }

    #[test]
    fn day1_part2_test() {
        let contents = fs::read_to_string("inputs/day1.txt").expect("couldn't read file");

        let result = day1_part2(&contents);
        assert_eq!(result, 1359);
    }
}
