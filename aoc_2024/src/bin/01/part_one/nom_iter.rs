use std::iter::zip;

use aocr_macro::aoc;
use nom::{
    character::complete::{self, line_ending, space1},
    combinator::{iterator, opt},
    sequence::{separated_pair, terminated},
    IResult,
};

#[aoc(part = "One", name = "nom_iter")]
pub fn solve_one(input: &str) -> i64 {
    let (_, (mut left, mut right)) = parse(input).unwrap();

    left.sort_unstable();
    right.sort_unstable();

    zip(left, right).map(|(l, r)| (l - r).abs()).sum()
}

fn parse(input: &str) -> IResult<&str, (Vec<i64>, Vec<i64>)> {
    let mut it = iterator(
        input,
        terminated(
            separated_pair(complete::i64, space1, complete::i64),
            opt(line_ending),
        ),
    );

    let parsed = it.collect::<(Vec<i64>, Vec<i64>)>();
    let res: IResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
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
