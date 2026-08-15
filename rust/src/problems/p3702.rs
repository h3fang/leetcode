pub struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        if nums.iter().all(|&x| x == 0) {
            return 0;
        }
        let n = nums.len() as i32;
        let xor = nums.iter().fold(0, |acc, x| acc ^ x);
        if xor == 0 { n - 1 } else { n }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::longest_subsequence(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::longest_subsequence(vec![2, 3, 4]));
    }
}
