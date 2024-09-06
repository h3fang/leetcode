pub struct Solution;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let i = coordinates.as_bytes()[0] - b'a';
        let j = coordinates.as_bytes()[1] - b'1';
        (i + j) % 2 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::square_is_white("a1".to_string()));
    }

    #[test]
    fn case2() {
        assert!(Solution::square_is_white("h3".to_string()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::square_is_white("c7".to_string()));
    }
}
