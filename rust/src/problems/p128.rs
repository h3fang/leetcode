use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().collect::<HashSet<i32>>();
        let mut result = 0;

        for &n in &nums {
            if !nums.contains(&(n - 1)) {
                let mut k = n + 1;
                while nums.contains(&k) {
                    k += 1;
                }
                result = result.max(k - n);
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
        assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            9,
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            4,
            Solution::longest_consecutive(vec![
                -7, -1, 3, -9, -4, 7, -3, 2, 4, 9, 4, -9, 8, -7, 5, -1, -7
            ])
        );
    }
}
