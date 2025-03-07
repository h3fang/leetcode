pub struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let n = n as i64;
        let (mut result, mut x, mut i) = (0, 1, 0);
        while x <= n {
            if i < nums.len() && nums[i] as i64 <= x {
                x += nums[i] as i64;
                i += 1;
            } else {
                x *= 2;
                result += 1;
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
        assert_eq!(1, Solution::min_patches(vec![1, 3], 6));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_patches(vec![1, 5, 10], 20));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::min_patches(vec![1, 2, 2], 5));
    }
}
