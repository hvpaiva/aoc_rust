use aocr_macro::aoc;

#[aoc(part = "One")]
pub fn solve_one(input: &str) -> i64 {
    input.chars().map(from).sum()
}

#[aoc(part = "Two", name = "closures")]
pub fn solve_two(input: &str) -> i64 {
    input
        .chars()
        .map(from)
        .enumerate()
        .scan(0, |count, (i, c)| {
            *count += c;
            Some((*count, i))
        })
        .find(|&(count, _)| count < 0)
        .map(|(_, i)| i as i64 + 1)
        .expect("Santa didn't reach the basement")
}

#[aoc(part = "Two", name = "for_loop")]
pub fn solve_two_for_loop(input: &str) -> i64 {
    let mut count = 0;
    for (i, c) in input.chars().enumerate() {
        count += from(c);
        if count < 0 {
            return i as i64 + 1;
        }
    }
    panic!("Santa didn't reach the basement")
}

fn from(ch: char) -> i64 {
    match ch {
        '(' => 1,
        ')' => -1,
        _ => panic!("Invalid character: {}", ch),
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(from('('), 1);
        assert_eq!(from(')'), -1);
    }

    #[test]
    #[should_panic(expected = "Invalid character: A")]
    fn test_from_panic() {
        assert_eq!(from('A'), 0);
    }

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn test_part_one(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(solve_one(input), expected);
    }

    #[rstest]
    #[case("example")]
    #[should_panic]
    fn test_part_one_panic(#[case] input: &str) {
        solve_one(input);
    }

    #[rstest]
    #[case("))(((((", 1)]
    #[case("())", 3)]
    #[case("(())())", 7)]
    fn test_part_two(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(solve_two(input), expected);
        assert_eq!(solve_two_for_loop(input), expected);
    }

    #[rstest]
    #[case("(())")]
    #[case("(((")]
    #[case("(()(()(")]
    #[should_panic(expected = "Santa didn't reach the basement")]
    fn test_part_two_panic(#[case] input: &str) {
        solve_two(input);
        solve_two_for_loop(input);
    }
}
