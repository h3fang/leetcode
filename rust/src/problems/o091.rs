pub struct Solution;

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut dp = [0; 3];
        for c in costs {
            let d0 = dp[1].min(dp[2]) + c[0];
            let d1 = dp[0].min(dp[2]) + c[1];
            let d2 = dp[0].min(dp[1]) + c[2];
            dp = [d0, d1, d2];
        }
        *dp.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let costs = [[17, 2, 17], [16, 16, 5], [14, 3, 19]];
        let costs = costs.iter().map(|c| c.to_vec()).collect();
        assert_eq!(10, Solution::min_cost(costs));
    }
}
