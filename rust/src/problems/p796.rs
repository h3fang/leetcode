pub struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        s.len() == goal.len() && s.repeat(2).contains(&goal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            true,
            Solution::rotate_string("abcde".to_string(), "cdeab".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            false,
            Solution::rotate_string("abcde".to_string(), "abced".to_string())
        );
    }
}
