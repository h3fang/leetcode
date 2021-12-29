pub struct Solution;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        let mut count = [0; 201];
        for b in (1..n - 2).rev() {
            for d in b + 2..n {
                let j = (nums[d] - nums[b + 1]) + 100;
                count[j as usize] += 1;
            }
            for a in 0..b {
                result += count[(nums[a] + nums[b] + 100) as usize];
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
        assert_eq!(4, Solution::count_quadruplets(vec![1, 1, 1, 3, 5]));
    }
}
