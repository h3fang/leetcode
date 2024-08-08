pub struct Solution;

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut f = vec![-1; n];
        f[0] = 0;
        for (i, &e) in nums.iter().enumerate() {
            if f[i] == -1 {
                continue;
            }
            for j in i + 1..n {
                if (nums[j] - e).abs() <= target {
                    f[j] = f[j].max(f[i] + 1);
                }
            }
        }
        f[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 3));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 0));
    }
}
