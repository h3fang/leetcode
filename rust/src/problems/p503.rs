pub struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![-1; n];
        let mut s = Vec::with_capacity(n);
        for (i, &x) in nums.iter().chain(&nums).enumerate() {
            while !s.is_empty() && x > nums[*s.last().unwrap()] {
                let j = s.pop().unwrap();
                result[j] = x;
            }
            s.push(i % n);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![2, -1, 2],
            Solution::next_greater_elements(vec![1, 2, 1])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![2, 3, 4, -1, 4],
            Solution::next_greater_elements(vec![1, 2, 3, 4, 3])
        );
    }
}
