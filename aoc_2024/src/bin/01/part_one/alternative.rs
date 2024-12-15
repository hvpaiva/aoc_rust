use aocr_macro::aoc;

#[aoc(part = "One", name = "alternative")]
pub fn solve_one(input: &str) -> i64 {
    let (mut left, mut right): (Vec<i64>, Vec<i64>) = input
        .lines()
        .map(|line| {
            let (l_str, r_str) = line.split_once("   ").unwrap();
            let l: i64 = l_str.parse().unwrap();
            let r: i64 = r_str.parse().unwrap();
            (l, r)
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(&right).map(|(l, r)| (l - r).abs()).sum()
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
