pub struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut dp = vec![vec![0; n + 1]; n];
        let mut sum = 0;
        for i in (0..n).rev() {
            sum += piles[i];
            for m in 1..=n {
                if i + 2 * m >= n {
                    dp[i][m] = sum;
                } else {
                    for x in 1..=2 * m {
                        dp[i][m] = dp[i][m].max(sum - dp[i + x][m.max(x)]);
                    }
                }
            }
        }
        dp[0][1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::stone_game_ii(vec![2, 7, 9, 4, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(104, Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]));
    }
}
