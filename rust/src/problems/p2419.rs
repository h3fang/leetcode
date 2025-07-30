pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let (mut ans, mut curr) = (0, 0);
        let max = *nums.iter().max().unwrap();
        for x in nums {
            if x == max {
                curr += 1;
                ans = ans.max(curr);
            } else {
                curr = 0;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::longest_subarray(vec![1, 2, 3, 3, 2, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::longest_subarray(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            1,
            Solution::longest_subarray(vec![
                96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 279979
            ])
        );
    }
}
