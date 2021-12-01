use itertools::Itertools;

fn day1_part1(input: &str) -> i32 {
    let (_, count): (i32, i32) = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .fold((i32::MAX, 0), |(prev, count), elem| (elem, if elem > prev { count + 1} else { count }));

    count
}

fn day1_part2(input: &str) -> i32 {
    let (_, count): (i32, i32) = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .tuple_windows::<(_, _, _)>()
        .fold((i32::MAX, 0), |(prev, count), (a, b, c)| (a + b + c, if a + b + c > prev { count + 1 } else { count }));

    count
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

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
