pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = nums[0];
        let mut votes = 0;
        for n in nums {
            if votes == 0 {
                candidate = n;
            }
            votes += if n == candidate { 1 } else { -1 };
        }
        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![3, 2, 3];
        assert_eq!(3, Solution::majority_element(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(2, Solution::majority_element(nums));
    }

    #[test]
    fn case3() {
        let nums = vec![6, 5, 5];
        assert_eq!(5, Solution::majority_element(nums));
    }
}
