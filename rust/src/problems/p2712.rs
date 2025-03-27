pub struct Solution;

impl Solution {
    pub fn minimum_cost(s: String) -> i64 {
        let n = s.len();
        let mut ans = 0;
        for (i, w) in s.as_bytes().windows(2).enumerate() {
            if w[0] != w[1] {
                ans += (i + 1).min(n - i - 1) as i64;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::minimum_cost("0011".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(9, Solution::minimum_cost("010101".to_string()));
    }
}
