use aocr_macro::aoc;

#[aoc(part = "One")]
pub fn solve_one(input: &str) -> i64 {
    let mut left = vec![];
    let mut right = vec![];
    for (a, b) in input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .map(parse_tuple)
    {
        left.push(a);
        right.push(b);
    }

    left.sort();
    right.sort();

    left.iter().zip(right).map(|(l, r)| (l - r).abs()).sum()
}

fn parse_tuple(s: (&str, &str)) -> (i64, i64) {
    (s.0.parse().unwrap(), s.1.parse().unwrap())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one_input() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

        let expected = 11;
        pretty_assertions::assert_eq!(super::solve_one(input), expected);
    }
}
