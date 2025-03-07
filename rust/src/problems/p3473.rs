pub struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32, m: i32) -> i32 {
        let (n, k, m) = (nums.len(), k as usize, m as usize);
        let mut s = Vec::with_capacity(n + 1);
        s.push(0);
        for &x in &nums {
            s.push(s.last().unwrap() + x);
        }
        let mut f = vec![vec![i32::MIN / 2; k + 1]; n + 1];
        f[0][0] = 0;
        let mut max = vec![vec![i32::MIN / 2; k + 1]; n + 1];
        for i in 1..=n {
            for j in 0..=k {
                f[i][j] = f[i - 1][j];
                if j > 0 && i >= m {
                    f[i][j] = f[i][j].max(max[i - m][j - 1]);
                }
                max[i][j] = f[i][j].max(max[i - 1][j] + nums[i - 1]);
            }
        }
        f[n][k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(13, Solution::max_sum(vec![1, 2, -1, 3, 3, 4], 2, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(-10, Solution::max_sum(vec![-10, 3, -1, -2], 4, 1));
    }
}
