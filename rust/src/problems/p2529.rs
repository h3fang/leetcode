pub struct Solution;

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let neg = nums.partition_point(|&x| x < 0);
        let pos = nums.len() - nums.partition_point(|&x| x <= 0);
        pos.max(neg) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::maximum_count(vec![5, 20, 66, 1314]));
    }
}
