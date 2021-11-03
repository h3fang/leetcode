pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();

        dp[0] = (0..word2.len() as i32 + 1).collect();

        for i in 1..=word1.len() {
            dp[i][0] = i as i32;
            for j in 1..=word2.len() {
                if w1[i - 1] == w2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j - 1]);
                }
            }
        }

        dp[word1.len()][word2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let word1 = "horse".to_string();
        let word2 = "ros".to_string();
        assert_eq!(3, Solution::min_distance(word1, word2));
    }

    #[test]
    fn case2() {
        let word1 = "intention".to_string();
        let word2 = "execution".to_string();
        assert_eq!(5, Solution::min_distance(word1, word2));
    }
}
