pub struct Solution;

impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut ans = 1;
        let mut min = nums[0];
        for x in nums {
            if x - min > k {
                ans += 1;
                min = x;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::partition_array(vec![3, 6, 1, 2, 5], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::partition_array(vec![1, 2, 3], 1));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::partition_array(vec![2, 2, 4, 5], 0));
    }
}
