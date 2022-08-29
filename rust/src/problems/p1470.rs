pub struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = vec![0; 2 * n];
        for i in 0..n {
            result[2 * i] = nums[i];
            result[2 * i + 1] = nums[n + i];
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
            vec![2, 3, 5, 4, 1, 7],
            Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3)
        );
    }
}
