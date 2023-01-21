pub struct Solution;

impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let mut dp = [1, 0, 1];
        for obstracle in obstacles {
            if obstracle > 0 {
                dp[obstracle as usize - 1] = i32::MAX / 2;
            }
            for i in 0..3_usize {
                if obstracle == i as i32 + 1 {
                    continue;
                }
                dp[i] = dp[i].min(dp[(i + 1) % 3].min(dp[(i + 2) % 3]) + 1)
            }
        }
        dp[0].min(dp[1].min(dp[2]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::min_side_jumps(vec![0, 1, 2, 3, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_side_jumps(vec![0, 1, 1, 3, 3, 0]));
    }
    #[test]
    fn case3() {
        assert_eq!(2, Solution::min_side_jumps(vec![0, 2, 1, 0, 3, 0]));
    }
}
