pub struct Solution;

impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right) = (*nums.iter().min().unwrap(), *nums.iter().max().unwrap());
        while left <= right {
            let m = (right - left) / 2 + left;
            let mut c = 0;
            let mut visited = false;
            for &x in &nums {
                if x <= m && !visited {
                    c += 1;
                    visited = true;
                } else {
                    visited = false;
                }
            }
            if c >= k {
                right = m - 1;
            } else {
                left = m + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::min_capability(vec![2, 3, 5, 9], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_capability(vec![2, 7, 9, 3, 1], 2));
    }
}
