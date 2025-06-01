pub struct Solution;

impl Solution {
    pub fn check_equal_partitions(nums: Vec<i32>, target: i64) -> bool {
        if nums.iter().any(|&x| x as i64 > target) {
            return false;
        }
        let n = nums.len();
        for m in 1u32..(2 << n) - 1 {
            let (mut p1, mut p2) = (1, 1);
            for (i, &x) in nums.iter().enumerate() {
                if m & (1 << i) > 0 {
                    p1 *= x as i64;
                } else {
                    p2 *= x as i64;
                }
                if p1 > target || p2 > target {
                    break;
                }
            }
            if p1 == target && p2 == target {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_equal_partitions(vec![3, 1, 6, 8, 4], 24));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_equal_partitions(vec![2, 5, 3, 7], 15));
    }
}
