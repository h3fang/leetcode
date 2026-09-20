pub struct Solution;

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.bytes()
            .enumerate()
            .map(|(i, b)| i32::from(26 - (b - b'a')) * (i as i32 + 1))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(148, Solution::reverse_degree("abc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(160, Solution::reverse_degree("zaza".to_string()));
    }
}
