pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![0; text2.len() + 1];

        let t1 = text1.as_bytes();
        let t2 = text2.as_bytes();

        for i in 1..=text1.len() {
            let mut pre = dp[0];
            for j in 1..=text2.len() {
                let curr = dp[j];
                if t1[i - 1] == t2[j - 1] {
                    dp[j] = pre + 1;
                } else {
                    dp[j] = dp[j].max(dp[j - 1]);
                }
                pre = curr;
            }
        }
        dp[text2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let text1 = "abcde".to_string();
        let text2 = "ace".to_string();
        assert_eq!(3, Solution::longest_common_subsequence(text1, text2));
    }

    #[test]
    fn case2() {
        let text1 = "abc".to_string();
        let text2 = "abc".to_string();
        assert_eq!(3, Solution::longest_common_subsequence(text1, text2));
    }

    #[test]
    fn case3() {
        let text1 = "abc".to_string();
        let text2 = "def".to_string();
        assert_eq!(0, Solution::longest_common_subsequence(text1, text2));
    }
}
