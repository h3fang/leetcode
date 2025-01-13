pub struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().map(|&x| x as i64).sum();
        let (mut left, mut result) = (0, 0);
        for &x in nums.iter().take(nums.len() - 1) {
            left += x as i64;
            if 2 * left >= sum {
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
        assert_eq!(2, Solution::ways_to_split_array(vec![10, 4, -8, 7]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::ways_to_split_array(vec![6, -1, 9]));
    }
}
