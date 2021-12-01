pub struct Solution;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let s = s.as_bytes();
        let mut prev = s[0];
        let mut curr = 1;
        let mut result = 1;
        for &e in &s[1..] {
            if e == prev {
                curr += 1;
            } else {
                result = result.max(curr);
                prev = e;
                curr = 1;
            }
        }
        result = result.max(curr);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::max_power("leetcode".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::max_power("abbcccddddeeeee".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::max_power("a".to_string()));
    }
}
