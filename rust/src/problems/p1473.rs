pub struct Solution;

impl Solution {
    pub fn min_cost(mut houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        houses.iter_mut().for_each(|h| *h -= 1);
        const INF: i32 = i32::MAX / 2;
        let m = m as usize;
        let n = n as usize;
        let target = target as usize;
        let mut dp = vec![vec![vec![INF; target]; n]; m];
        for i in 0..m {
            for j in 0..n {
                if houses[i] != -1 && houses[i] != j as i32 {
                    continue;
                }
                for k in 0..target {
                    for j0 in 0..n {
                        if j == j0 {
                            if i == 0 {
                                if k == 0 {
                                    dp[i][j][k] = 0;
                                }
                            } else {
                                dp[i][j][k] = dp[i][j][k].min(dp[i - 1][j][k]);
                            }
                        } else if i > 0 && k > 0 {
                            dp[i][j][k] = dp[i][j][k].min(dp[i - 1][j0][k - 1]);
                        }
                    }
                    if dp[i][j][k] < INF && houses[i] == -1 {
                        dp[i][j][k] += cost[i][j];
                    }
                }
            }
        }
        let mut result = INF;
        #[allow(clippy::needless_range_loop)]
        for j in 0..n {
            result = result.min(dp[m - 1][j][target - 1]);
        }
        if result == INF { -1 } else { result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let houses = vec![0, 0, 0, 0, 0];
        let cost = [[1, 10], [10, 1], [10, 1], [1, 10], [5, 1]];
        let cost = cost.iter().map(|c| c.to_vec()).collect();
        let m = 5;
        let n = 2;
        let target = 3;
        assert_eq!(9, Solution::min_cost(houses, cost, m, n, target));
    }

    #[test]
    fn case2() {
        let houses = vec![0, 2, 1, 2, 0];
        let cost = [[1, 10], [10, 1], [10, 1], [1, 10], [5, 1]];
        let cost = cost.iter().map(|c| c.to_vec()).collect();
        let m = 5;
        let n = 2;
        let target = 3;
        assert_eq!(11, Solution::min_cost(houses, cost, m, n, target));
    }

    #[test]
    fn case3() {
        let houses = vec![3, 1, 2, 3];
        let cost = [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]];
        let cost = cost.iter().map(|c| c.to_vec()).collect();
        let m = 4;
        let n = 3;
        let target = 3;
        assert_eq!(-1, Solution::min_cost(houses, cost, m, n, target));
    }
}
