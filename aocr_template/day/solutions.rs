use aocr_macro::aoc;

#[aoc(part = "One")]
pub fn solve_one(input: &str) -> i64 {
    input.lines().count() as i64
}

#[aoc(part = "Two")]
pub fn solve_two(input: &str) -> i64 {
    input.len() as i64
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
}
