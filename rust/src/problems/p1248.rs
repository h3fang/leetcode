pub struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut idx = Vec::with_capacity(nums.len() + 2);
        idx.push(-1);
        for (i, x) in nums.iter().enumerate() {
            if x % 2 == 1 {
                idx.push(i as i32);
            }
        }
        idx.push(nums.len() as i32);
        let k = k as usize;
        if idx.len() < k {
            return 0;
        }
        let mut result = 0;
        for i in 1..idx.len() - k {
            result += (idx[i + k] - idx[i + k - 1]) * (idx[i] - idx[i - 1]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::number_of_subarrays(vec![2, 4, 6], 1));
    }

    #[test]
    fn case3() {
        assert_eq!(
            16,
            Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2)
        );
    }
}
