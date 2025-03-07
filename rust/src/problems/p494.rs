pub struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        let d = sum - target;
        if d < 0 || d % 2 != 0 {
            return 0;
        }
        let neg = d / 2;
        let mut f = vec![0; neg as usize + 1];
        f[0] = 1;
        for &x in &nums {
            for j in (x..=neg).rev() {
                f[j as usize] += f[(j - x) as usize];
            }
        }
        f[neg as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::find_target_sum_ways(vec![1], 1));
    }
}
