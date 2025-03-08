pub struct Solution;

impl Solution {
    pub fn largest_integer(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if k == 1 {
            nums.sort_unstable_by_key(|&x| -x);
            for i in 0..n {
                if (i == 0 || nums[i - 1] != nums[i]) && (i == n - 1 || nums[i] != nums[i + 1]) {
                    return nums[i];
                }
            }
            return -1;
        }
        if k as usize == n {
            return nums.into_iter().max().unwrap();
        }
        let mut max = -1;
        if !nums[1..].iter().any(|e| *e == nums[0]) {
            max = max.max(nums[0]);
        }
        if !nums[..n - 1].iter().any(|e| *e == nums[n - 1]) {
            max = max.max(nums[n - 1]);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, Solution::largest_integer(vec![3, 9, 2, 1, 7], 3))
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::largest_integer(vec![3, 9, 7, 2, 1, 7], 4))
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::largest_integer(vec![0, 0], 1))
    }
}
