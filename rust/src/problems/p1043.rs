pub struct Solution;

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let k = k as usize;
        let mut dp = vec![0; n + 1];
        for i in 1..=n {
            let mut max = arr[i - 1];
            for j in (i.saturating_sub(k)..i).rev() {
                dp[i] = dp[i].max(dp[j] + max * (i - j) as i32);
                if j > 0 {
                    max = max.max(arr[j - 1]);
                }
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            84,
            Solution::max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            83,
            Solution::max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::max_sum_after_partitioning(vec![1], 1));
    }
}
