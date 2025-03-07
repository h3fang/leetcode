pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max = *nums.iter().max().unwrap();
        let (mut c, mut left, mut result) = (0, 0, 0);
        for &x in nums.iter() {
            if x == max {
                c += 1;
            }
            while c == k {
                if nums[left] == max {
                    c -= 1;
                }
                left += 1;
            }
            result += left;
        }
        result as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_subarrays(vec![1, 4, 2, 1], 3));
    }
}
