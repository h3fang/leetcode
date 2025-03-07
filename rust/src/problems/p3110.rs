pub struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .map(|w| w[0].abs_diff(w[1]) as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(13, Solution::score_of_string("hello".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(50, Solution::score_of_string("zaz".to_string()));
    }
}
