pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        nums.sort_unstable();
        nums.dedup();
        let (mut l, mut points) = (0, 0);
        for (r, &b) in nums.iter().enumerate() {
            while nums[l] < b - n + 1 {
                l += 1;
            }
            points = points.max(r - l + 1);
        }
        n - points as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::min_operations(vec![4, 2, 5, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::min_operations(vec![1, 2, 3, 5, 6]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::min_operations(vec![1, 10, 100, 1000]));
    }
}
