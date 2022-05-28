pub struct Solution;

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        (s.chars().filter(|c| *c == letter).count() * 100 / s.len()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(33, Solution::percentage_letter("foobar".into(), 'o'));
    }
}
