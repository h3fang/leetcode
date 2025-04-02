pub struct Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let (mut ans, mut max, mut max_d) = (0, 0, 0);
        for x in nums {
            ans = ans.max(max_d * x as i64);
            max_d = max_d.max(max - x as i64);
            max = max.max(x as i64);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(77, Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]));
    }

    #[test]
    fn case2() {
        assert_eq!(133, Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::maximum_triplet_value(vec![1, 2, 3]));
    }
}
