pub struct Solution;

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let n = scores.len();
        let mut ids = (0..n).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|&i| (ages[i], scores[i]));
        let mut dp = vec![0; n];
        let mut result = 0;
        for (i, &id) in ids.iter().enumerate() {
            dp[i] = scores[id];
            for j in 0..i {
                let last = ids[j];
                if scores[last] <= scores[id] {
                    dp[i] = dp[i].max(dp[j] + scores[id]);
                }
            }
            result = result.max(dp[i]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            34,
            Solution::best_team_score(vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            16,
            Solution::best_team_score(vec![4, 5, 6, 5], vec![2, 1, 2, 1])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            6,
            Solution::best_team_score(vec![1, 2, 3, 5], vec![8, 9, 10, 1])
        );
    }
}
