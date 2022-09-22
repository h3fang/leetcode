pub struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let s = nums.iter().sum::<i32>();
        if s % k != 0 {
            return false;
        }
        let p = s / k;
        nums.sort_unstable();
        if *nums.last().unwrap() > p {
            return false;
        }
        let mut dp = vec![false; 1 << n];
        let mut sum = vec![0; 1 << n];
        dp[0] = true;
        for s in 0..(1 << n) {
            if !dp[s] {
                continue;
            }
            for (i, e) in nums.iter().enumerate() {
                if sum[s] + e > p {
                    break;
                }
                if (s >> i) & 1 == 0 {
                    let s1 = s | (1 << i);
                    if !dp[s1] {
                        sum[s1] = (sum[s] + e) % p;
                        dp[s1] = true;
                    }
                }
            }
        }
        dp[(1 << n) - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            true,
            Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            false,
            Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3)
        );
    }
}
