pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = vec![0; *days.last().unwrap() as usize + 1];
        let m = dp.len();
        let days: HashSet<i32> = days.into_iter().collect();
        for i in 1..m as i32 {
            dp[i as usize] = if days.contains(&i) {
                (dp[(i - 1).max(0) as usize] + costs[0]).min(
                    (dp[(i - 7).max(0) as usize] + costs[1])
                        .min(dp[(i - 30).max(0) as usize] + costs[2]),
                )
            } else {
                dp[i as usize - 1]
            }
        }
        dp[m - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            11,
            Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            17,
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15])
        );
    }
}
