pub struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n <= 1 {
            return 0;
        }
        let mut f = vec![vec![0; n + 1]; k as usize + 1];
        let mut result = 0;
        for i in 1..=k as usize {
            let mut prev_max = f[i - 1][0] - prices[0];
            for (j, p) in prices.iter().enumerate().take(n).skip(1) {
                f[i][j] = f[i][j - 1].max(p + prev_max);
                prev_max = prev_max.max(f[i - 1][j] - p);
                result = result.max(f[i][j]);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::max_profit(2, vec![2, 4, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::max_profit(7, vec![3, 2, 6, 5, 0, 3]));
    }
}
