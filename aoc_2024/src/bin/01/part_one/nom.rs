use std::iter::zip;

use aocr_macro::aoc;
use nom::{
    character::complete::{self, line_ending, space1},
    combinator::opt,
    multi::fold_many1,
    sequence::{separated_pair, terminated},
    IResult,
};

#[aoc(part = "One", name = "nom")]
pub fn solve_one(input: &str) -> i64 {
    let (_, (mut left, mut right)) = parse(input).unwrap();

    left.sort_unstable();
    right.sort_unstable();

    zip(left, right).map(|(l, r)| (l - r).abs()).sum()
}

fn parse(input: &str) -> IResult<&str, (Vec<i64>, Vec<i64>)> {
    fold_many1(
        terminated(
            separated_pair(complete::i64, space1, complete::i64),
            opt(line_ending),
        ),
        || (Vec::new(), Vec::new()),
        |mut acc: (Vec<i64>, Vec<i64>), (l, r)| {
            acc.0.push(l);
            acc.1.push(r);
            acc
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::solve_one;

    #[test]
    fn test_part_one_input() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

        let expected = 11;
        pretty_assertions::assert_eq!(solve_one(input), expected);
    }
}
