pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let mut dp = (0..w2.len() as i32 + 1).collect::<Vec<_>>();
        for i in 1..=w1.len() {
            let mut pre = dp[0];
            dp[0] += 1;
            for j in 1..=w2.len() {
                let curr = dp[j];
                if w1[i - 1] == w2[j - 1] {
                    dp[j] = pre;
                } else {
                    dp[j] = 1 + dp[j].min(dp[j - 1]);
                }
                pre = curr;
            }
        }

        dp[w2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let word1 = "sea".to_string();
        let word2 = "eat".to_string();
        assert_eq!(2, Solution::min_distance(word1, word2));
    }

    #[test]
    fn case2() {
        let word1 = "leetcode".to_string();
        let word2 = "etco".to_string();
        assert_eq!(4, Solution::min_distance(word1, word2));
    }
}
