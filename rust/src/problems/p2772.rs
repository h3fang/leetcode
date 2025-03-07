pub struct Solution;

impl Solution {
    pub fn check_array(mut nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let k = k as usize;
        for i in (0..n).rev() {
            if nums[i] == 0 {
                continue;
            }
            if nums[i] < 0 {
                return false;
            }
            if i < k - 1 {
                return false;
            }
            let a = nums[i];
            for e in &mut nums[i + 1 - k..=i] {
                *e -= a;
                if *e < 0 {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_array(vec![2, 2, 3, 1, 1, 0], 3));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_array(vec![1, 3, 1, 1], 2));
    }
}
